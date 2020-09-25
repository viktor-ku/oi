use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Timer {
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
    base_dir: PathBuf,
    cwd: PathBuf,
}

#[derive(Debug)]
pub struct TimerInput {
    pub start: i64,
    pub message: String,
    pub duration: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diary {
    entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    start: i64,
    message: String,
    duration: u64,
}

impl Entry {
    pub fn end(&self) -> i64 {
        self.start + (self.duration as i64)
    }
}

impl TimersStore {
    pub async fn new(base_dir: PathBuf) -> Self {
        let cwd = base_dir.join("timers");
        fs::create_dir_all(&cwd).await.unwrap();
        Self { cwd, base_dir }
    }

    pub async fn find_all(&self) -> Vec<Timer> {
        let mut v = Vec::new();
        let mut files = fs::read_dir(&self.cwd).await.unwrap();

        let mut maybe_file = files.next_entry().await.unwrap();
        while let Some(file) = maybe_file {
            let filepath = file.path();
            let content = fs::read_to_string(filepath).await.unwrap();
            let diary: Diary = toml::from_str(&content).unwrap();

            let entry = diary.entries.first().unwrap();

            v.push(Timer {
                start: entry.start,
                message: entry.message.clone(),
                duration: entry.duration,
            });

            maybe_file = files.next_entry().await.unwrap();
        }

        v
    }

    pub async fn find_active(&self) -> Vec<Timer> {
        let mut v = Vec::new();
        let mut files = fs::read_dir(&self.cwd).await.unwrap();

        let mut maybe_file = files.next_entry().await.unwrap();
        while let Some(file) = maybe_file {
            let filepath = file.path();
            let content = fs::read_to_string(filepath).await.unwrap();
            let diary: Diary = toml::from_str(&content).unwrap();

            let entry = diary.entries.first().unwrap();

            let timer = Timer {
                start: entry.start,
                message: entry.message.clone(),
                duration: entry.duration,
            };

            if timer.is_active() {
                v.push(timer);
            }

            maybe_file = files.next_entry().await.unwrap();
        }

        v
    }

    pub async fn create(&self, payload: TimerInput) -> Uuid {
        let id = Uuid::new_v4();
        let filename = self
            .cwd
            .join(id.to_hyphenated().to_string())
            .with_extension("toml");
        let mut file = fs::File::create(&filename)
            .await
            .expect("Could not create a record file!");
        let body = toml::to_string(&Diary {
            entries: vec![Entry {
                start: payload.start,
                message: payload.message,
                duration: payload.duration,
            }],
        })
        .unwrap();
        file.write_all(&body.as_bytes())
            .await
            .expect("Could not write to a record file");
        id
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub timers: TimersStore,
}

impl Store {
    pub async fn new(base_dir: PathBuf, sandbox: String) -> Self {
        let store_dir = base_dir.join(".store");
        let with_sandbox = store_dir.join(&sandbox);

        Self {
            timers: TimersStore::new(with_sandbox).await,
        }
    }
}
