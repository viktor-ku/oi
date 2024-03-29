use anyhow::Result;
use automerge::{transaction::Transactable, Automerge, AutomergeError, ObjId, ROOT};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use tokio::fs;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct Timer {
    pub uuid: Uuid,
    pub message: String,
    /// duration of the timeout in milliseconds
    pub duration: u64,
    /// start of the timeout in milliseconds (UTC)
    pub start: u64,
}

impl Timer {
    /// Amount of `ms` until the given timer will timeout
    pub fn remaining(&self, baseline: Option<u64>) -> u64 {
        let now = baseline.unwrap_or_else(|| chrono::Utc::now().timestamp_millis() as u64);
        let end = self.start + self.duration;
        if end > now {
            end - now
        } else {
            0
        }
    }

    pub fn is_active(&self, baseline: Option<u64>) -> bool {
        self.remaining(baseline) > 0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TimerInput {
    pub message: String,
    /// start of the timeout in milliseconds (UTC)
    pub start: u64,
    /// duration of the timeout in milliseconds
    pub duration: u64,
}

/// Even though it has "store" in its name, really this
/// is just a bunch of convenience functions that operate on
/// some given state. It should store bare minimum, as I'd
/// like to construct a bunch of these where needed with
/// minimum implications. Note, that it has no idea how this
/// state got here, maybe it was just constructed and passed,
/// or maybe it is behind a mutex that got locked and passed.
#[derive(Debug)]
pub struct TimersStore<'store> {
    pub state: &'store mut State,
}

impl<'store> TimersStore<'store> {
    pub fn new(state: &'store mut State) -> Self {
        Self { state }
    }

    pub fn find_all(&self) -> Result<Vec<Timer>> {
        let (_, timers) = self.state.0.get(ROOT, "timers")?.unwrap();

        let mut v = Vec::new();
        for (_, _, timer_id) in self.state.0.map_range(timers, ..) {
            let timer = self.try_assemble_timer(&timer_id)?;
            v.push(timer);
        }

        Ok(v)
    }

    pub fn find_by_uuid(&self, uuid: &Uuid) -> Result<Option<Timer>> {
        let (_, timers) = self.state.0.get(ROOT, "timers")?.unwrap();

        match self.state.0.get(timers, uuid.to_string())? {
            Some((_, timer_id)) => {
                let timer = self.try_assemble_timer(&timer_id)?;
                Ok(Some(timer))
            }
            None => Ok(None),
        }
    }

    /// Finds active timers only.
    ///
    /// Additionaly you can provide `baseline` which is a utc timestamp
    /// in milliseconds against which active timers will be compared.
    /// Compares to `now` otherwise.
    pub fn find_active(&self, baseline: Option<u64>) -> Result<Vec<Timer>> {
        let mut v = Vec::new();
        let (_, timers) = self.state.0.get(ROOT, "timers")?.unwrap();

        for (_, _, timer_id) in self.state.0.map_range(timers, ..) {
            let timer = self.try_assemble_timer(&timer_id)?;

            if timer.is_active(baseline) {
                v.push(timer);
            }
        }

        Ok(v)
    }

    pub fn delete_by_uuid(&mut self, uuid: &Uuid) -> Result<bool> {
        self.state
            .0
            .transact::<_, _, AutomergeError>(|tx| {
                let (_, timers) = tx.get(ROOT, "timers")?.unwrap();
                tx.delete(timers, uuid.to_string())?;
                Ok(())
            })
            .unwrap()
            .result;

        Ok(true)
    }

    pub fn delete_active(&mut self) -> Result<bool> {
        let active = self.find_active(None)?;

        self.state
            .0
            .transact::<_, _, AutomergeError>(|tx| {
                let (_, timers) = tx.get(ROOT, "timers")?.unwrap();
                for timer in active {
                    tx.delete(&timers, timer.uuid.to_string())?;
                }
                Ok(())
            })
            .unwrap()
            .result;

        Ok(true)
    }

    pub fn create(&mut self, payload: TimerInput) -> Result<Timer> {
        let uuid = Uuid::new_v4();

        self.state
            .0
            .transact::<_, _, AutomergeError>(|tx| {
                let (_, timers) = tx.get(ROOT, "timers")?.unwrap();
                let timer = tx.put_object(timers, uuid.to_string(), automerge::ObjType::Map)?;
                tx.put(&timer, "uuid", uuid.to_string())?;
                tx.put(&timer, "start", payload.start)?;
                tx.put(&timer, "message", payload.message.clone())?;
                tx.put(&timer, "duration", payload.duration)?;
                Ok(())
            })
            .unwrap()
            .result;

        Ok(Timer {
            uuid,
            start: payload.start,
            duration: payload.duration,
            message: payload.message,
        })
    }

    fn try_assemble_timer(&self, id: &ObjId) -> Result<Timer> {
        let state = &self.state.0;
        let uuid = {
            let (uuid, _) = state.get(id, "uuid")?.unwrap();
            let uuid = uuid.into_string().unwrap();
            Uuid::from_str(&uuid).unwrap()
        };
        let start = {
            let (start, _) = state.get(id, "start")?.unwrap();
            start.to_u64().unwrap()
        };
        let message = {
            let (message, _) = state.get(id, "message")?.unwrap();
            message.into_string().unwrap()
        };
        let duration = {
            let (duration, _) = state.get(id, "duration")?.unwrap();
            duration.to_u64().unwrap()
        };
        Ok(Timer {
            uuid,
            start,
            message,
            duration,
        })
    }
}

#[derive(Debug)]
pub struct State(pub Automerge);

impl State {
    pub async fn new(root: Option<PathBuf>) -> Result<Self> {
        match root {
            Some(root) => State::persist(root).await,
            None => Ok(State::in_memory()),
        }
    }

    pub fn in_memory() -> Self {
        Self(State::init())
    }

    pub async fn persist(root: PathBuf) -> Result<Self> {
        let dir = root.join(".store");
        let path = dir.join("data.bin");

        fs::create_dir_all(dir).await?;

        let state = match fs::read(&path).await {
            Ok(data) => match Automerge::load(&data) {
                Ok(val) => val,
                Err(_) => State::init(),
            },
            Err(_) => State::init(),
        };

        Ok(Self(state))
    }

    pub async fn save(&mut self, root: PathBuf) -> Result<()> {
        let path = root.join(".store").join("data.bin");
        let buf = self.0.save();
        fs::write(path, buf).await.unwrap();
        Ok(())
    }

    fn init() -> Automerge {
        let mut doc = Automerge::new();

        doc.transact::<_, _, AutomergeError>(|draft| {
            draft.put_object(ROOT, "timers", automerge::ObjType::Map)?;
            Ok(())
        })
        .expect("could not write the initial state")
        .result;

        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_find_all() -> Result<()> {
        let mut state = State::in_memory();
        let mut store = TimersStore::new(&mut state);

        store.create(TimerInput {
            start: 0,
            message: "some".into(),
            duration: 1000,
        })?;

        assert_eq!(store.find_all()?.len(), 1);

        Ok(())
    }

    #[test]
    fn test_find_active() -> Result<()> {
        let mut state = State::in_memory();
        let mut store = TimersStore::new(&mut state);

        store.create(TimerInput {
            start: 0,
            message: "some".into(),
            duration: 1000,
        })?;
        store.create(TimerInput {
            start: chrono::Utc::now().timestamp_millis() as _,
            message: "sometime in the future".into(),
            duration: 60_000,
        })?;
        assert_eq!(store.find_active(None)?.len(), 1);
        Ok(())
    }

    #[test]
    fn test_find_by_uuid() -> Result<()> {
        let mut state = State::in_memory();
        let mut store = TimersStore::new(&mut state);

        let timer = store.create(TimerInput {
            start: 0,
            message: "some".into(),
            duration: 1000,
        })?;
        assert_eq!(
            store.find_by_uuid(&timer.uuid)?,
            Some(Timer {
                uuid: timer.uuid.clone(),
                start: 0,
                message: "some".to_string(),
                duration: 1000
            })
        );
        assert_eq!(store.find_by_uuid(&Uuid::new_v4())?, None);
        Ok(())
    }

    #[test]
    fn test_delete() -> Result<()> {
        let mut state = State::in_memory();
        let mut store = TimersStore::new(&mut state);

        let timer = store.create(TimerInput {
            start: 0,
            message: "some".into(),
            duration: 1000,
        })?;

        assert_eq!(store.find_all()?.len(), 1);

        assert_eq!(store.delete_by_uuid(&timer.uuid)?, true);

        assert_eq!(store.find_by_uuid(&timer.uuid)?, None);
        assert_eq!(store.find_all()?.len(), 0);

        Ok(())
    }
}
