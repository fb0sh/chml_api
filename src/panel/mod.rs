mod function;
mod schema;

use crate::{ChmlApi, res::ApiResult};

impl ChmlApi {
    /// panel info
    pub async fn panelinfo(&self) -> ApiResult<schema::PanelInfo> {
        function::get_panel_info(self).await
    }
    /// server_status
    pub async fn server_status(&self) -> Result<schema::ServerMetrics, crate::res::ApiError> {
        function::get_api_server_status(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn test_panel_info() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.panelinfo().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_server_status() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.server_status().await.unwrap();
        println!("{:?}", res);
    }
}
