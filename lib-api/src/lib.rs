use reqwest::{Response, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTimer {
    pub start: i64,
    pub duration: u64,
    pub message: String,
}

#[derive(Debug)]
pub struct TimersResource<'resource> {
    client: reqwest::Client,
    url: &'resource str,
}

impl<'resource> TimersResource<'resource> {
    pub fn new(url: &'resource str) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
        }
    }

    pub async fn create(&self, body: &CreateTimer) -> Result<Response> {
        self.client
            .post(&format!("{}/timer", self.url))
            .json(body)
            .send()
            .await
    }
}

#[derive(Debug)]
pub struct Client<'api> {
    base_url: &'api str,
    pub timers: TimersResource<'api>,
}

impl<'api> Client<'api> {
    pub fn new(base_url: &'api str) -> Self {
        Self {
            base_url,
            timers: TimersResource::new(base_url),
        }
    }
}
