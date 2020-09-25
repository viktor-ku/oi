use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
    pub status: u16,
}

impl<T> Response<T> {
    #[inline]
    pub fn status(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap()
    }
}
