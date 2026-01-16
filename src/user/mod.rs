pub mod function;
pub mod schema;

use crate::{ChmlApi, res::ApiResult};

// 对外暴露的接口
impl ChmlApi {
    /// login & set token
    pub async fn login(
        &mut self,
        login_params: &function::LoginParams,
    ) -> ApiResult<schema::UserInfo> {
        function::login(self, login_params).await
    }

    /// send email code
    pub async fn send_email_code(
        &self,
        sec_params: &function::SendEmailCodeParams,
    ) -> ApiResult<()> {
        function::send_email_code(self, sec_params).await
    }

    /// register
    pub async fn register(&self, register_params: &function::RegisterParams) -> ApiResult<()> {
        function::register(self, register_params).await
    }

    /// user info
    pub async fn user_info(&self) -> ApiResult<schema::UserInfo> {
        function::get_user_info(&self).await
    }

    /// retoken
    pub async fn retoken(&self, reset_token_params: &function::ResetTokenParams) -> ApiResult<()> {
        function::reset_token(self, reset_token_params).await
    }

    /// qiandao
    pub async fn qiandao(&self, checkin_params: &function::CheckinParams) -> ApiResult<()> {
        function::checkin(self, checkin_params).await
    }

    /// reset_password
    pub async fn reset_password(
        &self,
        reset_password_params: &function::ResetPasswordParams,
    ) -> ApiResult<()> {
        function::reset_password(self, reset_password_params).await
    }

    /// update_username
    pub async fn update_username(&self, new_username: &str) -> ApiResult<()> {
        function::update_username(self, new_username).await
    }

    /// update_qq
    pub async fn update_qq(&self, new_qq: &str) -> ApiResult<()> {
        function::update_qq(self, new_qq).await
    }

    /// update_userimg
    pub async fn update_userimg(&self, new_user_img_url: &str) -> ApiResult<()> {
        function::update_user_img_url(self, new_user_img_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn test_login_success() {
        init_logger();

        let mut chml_api = ChmlApi::from_env().unwrap();
        let login_params = function::LoginParams {
            username: "username".to_string(),
            password: "password".to_string(),
        };
        let res = chml_api.login(&login_params).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_user_info() {
        init_logger();

        let mut chml_api = ChmlApi::from_env().unwrap();
        let login_params = function::LoginParams {
            username: "username".to_string(),
            password: "password".to_string(),
        };
        let _ = chml_api.login(&login_params).await;
        let res = chml_api.user_info().await;
        println!("{:?}", res);
    }
}
