use lib_store::Timer;
use lib_store::TimerInput;
use reqwest::Result;
use uuid::Uuid;

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

    pub async fn find_active(&self) -> Result<Vec<Timer>> {
        let res = self
            .client
            .get(&format!("{}/timers/active", self.url))
            .send()
            .await?;

        res.json().await
    }

    pub async fn find_all(&self) -> Result<Vec<Timer>> {
        let res = self
            .client
            .get(&format!("{}/timers", self.url))
            .send()
            .await?;

        res.json().await
    }

    pub async fn find_by_uuid(&self, uuid: &Uuid) -> Result<Option<Timer>> {
        let res = self
            .client
            .get(&format!("{}/timers/{}", self.url, uuid))
            .send()
            .await?;

        res.json().await
    }

    pub async fn delete_by_uuid(&self, uuid: &Uuid) -> Result<()> {
        self.client
            .delete(&format!("{}/timers/{}", self.url, uuid))
            .send()
            .await?;

        Ok(())
    }

    pub async fn create(&self, body: &TimerInput) -> Result<Timer> {
        let res = self
            .client
            .post(&format!("{}/timer", self.url))
            .json(body)
            .send()
            .await?;

        res.json().await
    }
}
