use crate::{ChmlApi, res::ApiResult};

mod function;
mod schema;

impl ChmlApi {
    /// user logs
    pub async fn get_user_logs(&self, page: i32, size: i32) -> ApiResult<function::UserLogsResult> {
        function::get_user_logs(self, page, size).await
    }

    /// messages
    pub async fn get_messages(
        &self,
        page: i32,
        size: i32,
        priority: i32,
    ) -> ApiResult<function::MessagesResult> {
        function::get_messages(self, page, size, priority).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn test_user_logs() {
        init_logger();
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.get_user_logs(1, 10).await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_messages() {
        init_logger();
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.get_messages(1, 10, 0).await.unwrap();
        println!("{:?}", res);
    }
}
