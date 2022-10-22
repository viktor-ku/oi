use anyhow::Result;
use automerge::{transaction::Transactable, Automerge, AutomergeError, ROOT};
use std::{path::PathBuf, str::FromStr};
use tokio::fs;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct Timer {
    pub uuid: uuid::Uuid,
    pub message: String,
    pub duration: u64,
    pub start: i64,
}

impl Timer {
    /// Amount of `ms` until the given timer will timeout
    pub fn remaining(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        let end = (self.start as u64) + self.duration;
        if end > now {
            end - now
        } else {
            0
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.remaining() > 0
    }
}

#[derive(Debug, Clone)]
pub struct TimersStore {
    pub state: Automerge,
}

#[derive(Debug)]
pub struct TimerInput {
    pub start: i64,
    pub message: String,
    pub duration: u64,
}

impl TimersStore {
    pub fn new(state: Automerge) -> Self {
        Self { state }
    }

    pub fn find_all(&self) -> Result<Vec<Timer>> {
        let mut v = Vec::new();

        let (_, timers) = self.state.get(ROOT, "timers")?.unwrap();

        for (_, _, timer) in self.state.map_range(timers, ..) {
            let uuid = {
                let (uuid, _) = self.state.get(&timer, "uuid")?.unwrap();
                let uuid = uuid.into_string().unwrap();
                Uuid::from_str(&uuid).unwrap()
            };
            let start = {
                let (start, _) = self.state.get(&timer, "start")?.unwrap();
                start.to_i64().unwrap()
            };
            let message = {
                let (message, _) = self.state.get(&timer, "message")?.unwrap();
                message.to_string()
            };
            let duration = {
                let (duration, _) = self.state.get(&timer, "duration")?.unwrap();
                duration.to_u64().unwrap()
            };
            let timer = Timer {
                uuid,
                start,
                message,
                duration,
            };
            v.push(timer);
        }

        Ok(v)
    }

    pub async fn find_by_uuid(&self, uuid: &uuid::Uuid) -> Result<Option<Timer>> {
        Ok(None)
    }

    pub async fn delete_by_uuid(&self, uuid: &uuid::Uuid) -> Result<Option<Uuid>> {
        Ok(None)
    }

    pub async fn find_active(&self) -> Result<Vec<Timer>> {
        Ok(Vec::new())
    }

    pub async fn create(&mut self, payload: TimerInput) -> Result<Uuid> {
        let uuid = Uuid::new_v4();

        self.state
            .transact::<_, _, AutomergeError>(|tx| {
                let (_, timers) = tx.get(ROOT, "timers")?.unwrap();
                let timer = tx.put_object(timers, uuid.to_string(), automerge::ObjType::Map)?;
                tx.put(&timer, "uuid", uuid.to_string())?;
                tx.put(&timer, "start", payload.start)?;
                tx.put(&timer, "message", payload.message)?;
                tx.put(&timer, "duration", payload.duration)?;
                Ok(())
            })
            .unwrap()
            .result;

        Ok(uuid)
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub timers: TimersStore,
}

impl Store {
    pub async fn new(base_dir: Option<PathBuf>) -> Result<Self> {
        let state = match base_dir {
            // persistence, please
            Some(base_dir) => {
                let store_dir = base_dir.join(".store");
                let store_path = store_dir.join("data.bin");

                fs::create_dir_all(store_dir).await?;
                let data = fs::read(store_path).await;

                match data {
                    Ok(data) => match Automerge::load(&data) {
                        Ok(val) => val,
                        Err(_) => Store::init(),
                    },
                    Err(_) => Store::init(),
                }
            }
            // im-memory only
            None => Store::init(),
        };

        Ok(Self {
            timers: TimersStore::new(state),
        })
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

    #[tokio::test]
    async fn a01() -> Result<()> {
        let mut store = Store::new(None).await?;
        store
            .timers
            .create(TimerInput {
                start: 0,
                message: "some".into(),
                duration: 1000,
            })
            .await?;
        assert_eq!(store.timers.find_all()?.len(), 1);
        Ok(())
    }
}
