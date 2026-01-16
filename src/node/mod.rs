pub mod function;
pub mod schema;

use crate::{ChmlApi, res::ApiResult};

impl ChmlApi {
    /// node list
    pub async fn node(&self) -> ApiResult<Vec<schema::Node>> {
        function::get_node_list(self).await
    }
    /// node detail
    pub async fn nodeinfo(&self, node_name: &str) -> ApiResult<schema::NodeInfo> {
        function::get_node_info(self, node_name).await
    }

    /// node stats
    pub async fn node_stats(&self) -> ApiResult<Vec<schema::NodeStats>> {
        function::get_node_stats(self).await
    }

    /// node uptime
    pub async fn node_uptime(
        &self,
        node_name: &str,
        days: i32,
    ) -> ApiResult<Vec<schema::NodeUptime>> {
        function::get_node_uptime(self, node_name, days).await
    }

    /// node status
    pub async fn node_status(&self, node_name: &str) -> ApiResult<function::NodeStatusResult> {
        function::get_node_status_info(self, node_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn test_node_list() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.node().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_node_detail() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.nodeinfo("南京电信-2").await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_node_stats() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.node_stats().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_node_uptime() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.node_uptime("南京电信-2", 7).await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_node_status() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.node_status("南京电信-2").await.unwrap();
        println!("{:?}", res);
    }
}
