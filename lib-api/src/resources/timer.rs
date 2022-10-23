use crate::impl_responder;
use crate::Response;
use lib_store::Timer;
use reqwest::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FindAllTimersResponse {
    pub timers: Vec<Timer>,
}
impl_responder!(FindAllTimersResponse);

#[derive(Debug, Serialize, Deserialize)]
pub struct FindActiveTimersResponse {
    pub timers: Vec<Timer>,
}
impl_responder!(FindActiveTimersResponse);

#[derive(Debug, Serialize, Deserialize)]
pub struct FindByUuidResponse {
    pub timer: Option<Timer>,
}
impl_responder!(FindByUuidResponse);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteByUuidResponse {
    pub uuid: Option<uuid::Uuid>,
}
impl_responder!(DeleteByUuidResponse);

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTimerInput {
    pub start: i64,
    pub duration: u64,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateTimerResponse {
    pub uuid: uuid::Uuid,
}
impl_responder!(CreateTimerResponse);

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

    pub async fn find_active(&self) -> Result<Response<Result<FindActiveTimersResponse>>> {
        let res = self
            .client
            .get(&format!("{}/timers/active", self.url))
            .send()
            .await;

        match res {
            Ok(res) => Ok(Response {
                status: res.status().as_u16(),
                data: res.json().await,
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn find_all(&self) -> Result<Response<Result<FindAllTimersResponse>>> {
        let res = self
            .client
            .get(&format!("{}/timers", self.url))
            .send()
            .await;

        match res {
            Ok(res) => Ok(Response {
                status: res.status().as_u16(),
                data: res.json().await,
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_uuid(
        &self,
        uuid: &uuid::Uuid,
    ) -> Result<Response<Result<FindByUuidResponse>>> {
        let res = self
            .client
            .get(&format!("{}/timers/{}", self.url, uuid))
            .send()
            .await;

        match res {
            Ok(res) => Ok(Response {
                status: res.status().as_u16(),
                data: res.json().await,
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_by_uuid(
        &self,
        uuid: &uuid::Uuid,
    ) -> Result<Response<Result<DeleteByUuidResponse>>> {
        let res = self
            .client
            .delete(&format!("{}/timers/{}", self.url, uuid))
            .send()
            .await;

        match res {
            Ok(res) => Ok(Response {
                status: res.status().as_u16(),
                data: res.json().await,
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn create(
        &self,
        body: &CreateTimerInput,
    ) -> Result<Response<Result<CreateTimerResponse>>> {
        let res = self
            .client
            .post(&format!("{}/timer", self.url))
            .json(body)
            .send()
            .await;

        match res {
            Ok(res) => Ok(Response {
                status: res.status().as_u16(),
                data: res.json().await,
            }),
            Err(e) => Err(e),
        }
    }
}
