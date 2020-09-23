use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Record {
    pub message: String,
    pub remaining: u64,
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
    pub fn end(&self) -> u64 {
        (self.start as u64) + self.duration * 1000
    }
}

impl RecordsStore {
    pub async fn new(base_dir: PathBuf) -> Self {
        let cwd = base_dir.join(".store/records");

        fs::create_dir_all(&cwd)
            .await
            .expect("Could not create a .store/records dir");

        Self { cwd, base_dir }
    }

    pub async fn find_active(&self) -> Vec<Record> {
        let mut v = Vec::new();
        let mut files = fs::read_dir(&self.cwd)
            .await
            .expect("Could not read .store/records dirs entries");

        let mut maybe_file = files.next_entry().await.unwrap();
        while let Some(file) = maybe_file {
            let filepath = file.path();
            let content = fs::read_to_string(filepath).await.unwrap();
            let diary: Diary = toml::from_str(&content).unwrap();

            let entry = diary.entries.first().unwrap();
            let now = chrono::Utc::now().timestamp_millis() as u64;

            if entry.end() > now {
                v.push(Record {
                    message: entry.message.clone(),
                    remaining: (entry.end() - now) / 1000,
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
    pub async fn new(base_dir: PathBuf) -> Self {
        Self {
            records: RecordsStore::new(base_dir).await,
        }
    }
}
