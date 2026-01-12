use super::schema::UserInfo;
use crate::prelude::*;

#[derive(Debug, Serialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

pub async fn login(chml_api: &ChmlApi, login_params: &LoginParams) -> Result<UserInfo, ApiError> {
    let api_url = chml_api.endpoint("/login");

    let req = chml_api.client.get(api_url).query(login_params).build()?;
    debug!("login request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<UserInfo>>()
        .await?;
    debug!("login response: {:?}", res);

    Ok(res.into_result()?)
}
