use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Record {
    pub message: String,
    pub remaining: i64,
    pub start: i64,
}

#[derive(Debug, Clone)]
pub struct RecordsStore {
    base_dir: PathBuf,
    cwd: PathBuf,
}

#[derive(Debug)]
pub struct CreateRecord {
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

impl RecordsStore {
    pub async fn new(base_dir: PathBuf) -> Self {
        let cwd = base_dir.join("records");
        fs::create_dir_all(&cwd).await.unwrap();
        Self { cwd, base_dir }
    }

    pub async fn find_all(&self) -> Vec<Record> {
        let mut v = Vec::new();
        let mut files = fs::read_dir(&self.cwd).await.unwrap();

        let mut maybe_file = files.next_entry().await.unwrap();
        while let Some(file) = maybe_file {
            let filepath = file.path();
            let content = fs::read_to_string(filepath).await.unwrap();
            let diary: Diary = toml::from_str(&content).unwrap();

            let entry = diary.entries.first().unwrap();
            let now = chrono::Utc::now().timestamp_millis();

            v.push(Record {
                start: entry.start,
                message: entry.message.clone(),
                remaining: (entry.end() - now),
            });

            maybe_file = files.next_entry().await.unwrap();
        }

        v
    }

    pub async fn find_active(&self) -> Vec<Record> {
        let mut v = Vec::new();
        let mut files = fs::read_dir(&self.cwd).await.unwrap();

        let mut maybe_file = files.next_entry().await.unwrap();
        while let Some(file) = maybe_file {
            let filepath = file.path();
            let content = fs::read_to_string(filepath).await.unwrap();
            let diary: Diary = toml::from_str(&content).unwrap();

            let entry = diary.entries.first().unwrap();
            let now = chrono::Utc::now().timestamp_millis();

            if entry.end() > now {
                v.push(Record {
                    start: entry.start,
                    message: entry.message.clone(),
                    remaining: (entry.end() - now),
                });
            }

            maybe_file = files.next_entry().await.unwrap();
        }

        v
    }

    pub async fn create(&self, payload: CreateRecord) -> Uuid {
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
    pub records: RecordsStore,
}

impl Store {
    pub async fn new(base_dir: PathBuf, sandbox: String) -> Self {
        let store_dir = base_dir.join(".store");
        let with_sandbox = store_dir.join(&sandbox);

        Self {
            records: RecordsStore::new(with_sandbox).await,
        }
    }
}
