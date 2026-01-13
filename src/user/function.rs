use super::schema::UserInfo;
use crate::prelude::*;

#[derive(Debug, Serialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

/// login & set token
pub async fn login(chml_api: &mut ChmlApi, login_params: &LoginParams) -> ApiResult<UserInfo> {
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

    // set token
    let user_info = res.as_result()?;
    debug!("set token: {}", &user_info.usertoken);
    chml_api.set_token(&user_info.usertoken);

    Ok(res)
}

#[derive(Debug, Serialize)]
pub struct SendEmailCodeParams {
    pub r#type: String,
    pub mail: String,
    pub lot_number: String,
    pub captcha_output: String,
    pub pass_token: Vec<String>,
    pub gen_time: String,
}
/// send email code
/// type 1: register 2: retoken
pub async fn send_email_code(
    chml_api: &ChmlApi,
    sec_params: &SendEmailCodeParams,
) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/sendmailcode");

    // post 携带query参数，很奇怪啊
    let req = chml_api.client.post(api_url).query(sec_params).build()?;
    debug!("send email code request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("send email code response: {:?}", res);

    Ok(res)
}

#[derive(Debug, Serialize)]
pub struct RegisterParams {
    pub username: String,
    pub password: String,
    pub mail: String,
    pub qq: i32,
    pub code: i32,
}
pub async fn register(chml_api: &ChmlApi, register_params: &RegisterParams) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/register");

    let req = chml_api
        .client
        .get(api_url)
        .query(register_params)
        .build()?;
    debug!("register request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("register response: {:?}", res);

    Ok(res)
}

pub async fn get_user_info(chml_api: &ChmlApi) -> ApiResult<UserInfo> {
    let api_url = chml_api.endpoint("/userinfo");

    let req = chml_api
        .client
        .get(api_url)
        .query(&[("token", chml_api.get_token()?)])
        .build()?;
    debug!("get user info request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<UserInfo>>()
        .await?;
    debug!("get user info response: {:?}", res);

    Ok(res)
}

#[derive(Debug, Serialize)]
pub struct ResetTokenParams {
    pub token: String,
    pub code: String,
}

pub async fn reset_token(
    chml_api: &ChmlApi,
    reset_token_params: &ResetTokenParams,
) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/retoken");

    let req = chml_api
        .client
        .get(api_url)
        .query(reset_token_params)
        .build()?;
    debug!("reset token request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("reset token response: {:?}", res);

    Ok(res)
}

#[derive(Debug, Serialize)]
pub struct CheckinParams {
    pub token: String,
    pub lot_number: String,
    pub captcha_output: String,
    pub pass_token: String,
    pub gen_time: String,
}
/// 希望API 还是规范一点
pub async fn checkin(chml_api: &ChmlApi, checkin_params: &CheckinParams) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/checkin");

    let req = chml_api.client.post(api_url).json(checkin_params).build()?;
    debug!("checkin request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("checkin response: {:?}", res);

    Ok(res)
}

#[derive(Debug, Serialize)]
pub struct ResetPasswordParams {
    pub token: String,
    pub original_password: String,
    pub new_password: String,
}

pub async fn reset_password(
    chml_api: &ChmlApi,
    reset_password_params: &ResetPasswordParams,
) -> ApiResult<()> {
    // 明明前几个接口还是 没下划线的，，，
    let api_url = chml_api.endpoint("/reset_password");

    let req = chml_api
        .client
        .get(api_url)
        .query(reset_password_params)
        .build()?;
    debug!("reset password request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("reset password response: {:?}", res);

    Ok(res)
}

pub async fn update_username(chml_api: &ChmlApi, new_username: &str) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/update_username");

    let req = chml_api
        .client
        .get(api_url)
        .query(&[
            ("token", chml_api.get_token()?),
            ("new_username", new_username),
        ])
        .build()?;
    debug!("update username request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("update username response: {:?}", res);

    Ok(res)
}

pub async fn update_qq(chml_api: &ChmlApi, new_qq: &str) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/update_qq");

    let req = chml_api
        .client
        .get(api_url)
        .query(&[("token", chml_api.get_token()?), ("new_qq", new_qq)])
        .build()?;
    debug!("update qq request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("update qq response: {:?}", res);

    Ok(res)
}

/// 注意，这个API会验证你的头像图片链接是否可以正常访问(获取请求头)，超时时间5秒。
/// 请确保您的图片链接支持他人访问，且可以正常访问。
/// API服务器位于中国宁波，请确定链接不屏蔽此地区。
pub async fn update_user_img_url(chml_api: &ChmlApi, new_user_img_url: &str) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/update_userimg");

    let req = chml_api
        .client
        .get(api_url)
        .query(&[
            ("token", chml_api.get_token()?),
            ("new_userimg", new_user_img_url),
        ])
        .build()?;
    debug!("update user img url request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("update user img url response: {:?}", res);

    Ok(res)
}

pub async fn get_messages(chml_api: &ChmlApi) {
    // API 文档没有写清楚，暂时不实现
    unimplemented!()
}
