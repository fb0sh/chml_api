use crate::ChmlApi;
mod function;
mod schema;

impl ChmlApi {
    // 对外暴露的接口
    pub async fn login(
        &self,
        login_params: &function::LoginParams,
    ) -> Result<schema::UserInfo, crate::res::ApiError> {
        function::login(self, login_params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn test_login_success() {
        init_logger();

        let chml_api = ChmlApi::new(crate::BASE_URL);
        let login_params = function::LoginParams {
            username: "username".to_string(),
            password: "password".to_string(),
        };
        let res = chml_api.login(&login_params).await;
        println!("{:?}", res);
    }
}
