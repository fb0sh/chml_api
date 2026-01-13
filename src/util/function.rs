use crate::{
    prelude::*,
    util::schema::{Message, UserLog},
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogsResult {
    total: i32,
    size: i32,
    totalPages: i32,
    page: i32,
    logs: Vec<UserLog>,
}

pub async fn get_user_logs(chml: &ChmlApi, page: i32, size: i32) -> ApiResult<UserLogsResult> {
    let api_url = chml.endpoint("/log/query");
    let req = chml
        .client
        .get(api_url)
        .header("Authorization", format!("Bearer {}", chml.get_token()?))
        .query(&[("page", &page.to_string()), ("size", &size.to_string())])
        .build()?;
    debug!("get user logs request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<UserLogsResult>>()
        .await?;
    debug!("get user logs response: {:?}", res);
    Ok(res)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagesResult {
    pub total: i32,
    pub size: i32,
    pub totalPages: i32,
    pub page: i32,
    pub messages: Vec<Message>,
}

/// get messages
/// page: page number
/// size: page size
pub async fn get_messages(
    chml: &ChmlApi,
    page: i32,
    size: i32,
    priority: i32,
) -> ApiResult<MessagesResult> {
    let api_url = chml.endpoint("/message/list");
    let req = chml
        .client
        .get(api_url)
        .header("Authorization", format!("Bearer {}", chml.get_token()?))
        .query(&[
            ("page", &page.to_string()),
            ("size", &size.to_string()),
            ("priority", &priority.to_string()),
        ])
        .build()?;
    debug!("get messages request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<MessagesResult>>()
        .await?;
    debug!("get messages response: {:?}", res);
    Ok(res)
}
