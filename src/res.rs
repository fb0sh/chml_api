use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub msg: String,
    pub code: u16,
    pub state: String,
    pub data: Option<T>,
}
impl<T> ApiResponse<T> {
    pub fn into_result(self) -> Result<T, ApiError> {
        if self.code != 200 || self.state != "success" {
            return Err(ApiError::Api {
                code: self.code,
                state: self.state,
                msg: self.msg,
            });
        }

        self.data.ok_or(ApiError::EmptyData)
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("api error: code={code}, state={state}, msg={msg}")]
    Api {
        code: u16,
        state: String,
        msg: String,
    },

    #[error("api returned success but data is null")]
    EmptyData,

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
