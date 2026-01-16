pub mod function;
pub mod schema;

use crate::{ChmlApi, res::ApiResult};

impl ChmlApi {
    /// get_tunnels
    pub async fn tunnel(&self) -> ApiResult<Vec<schema::Tunnel>> {
        function::get_tunnels(self).await
    }

    /// create_tunnel
    pub async fn create_tunnel(
        &self,
        params: &function::CreateTunnelParams,
    ) -> ApiResult<schema::Tunnel> {
        function::create_tunnel(self, params).await
    }

    /// delete_tunnel
    pub async fn delete_tunnel(&self, tunnel_id: &str) -> ApiResult<()> {
        function::delete_tunnel(self, tunnel_id).await
    }

    /// update_tunnel
    pub async fn update_tunnel(
        &self,
        tunnel_update: schema::TunnelUpdate,
    ) -> ApiResult<schema::Tunnel> {
        function::update_tunnel(self, tunnel_update).await
    }

    /// tunnel_config
    pub async fn tunnel_config(&self, node: &str, tunnel_names: &[&str]) -> ApiResult<String> {
        function::get_tunnel_config(self, node, tunnel_names).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    #[tokio::test]
    async fn test_tunnel() {
        init_logger();
        let chml_api = ChmlApi::from_env().unwrap();

        let res = chml_api.tunnel().await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_create_tunnel() {
        init_logger();
        let chml_api = ChmlApi::from_env().unwrap();

        // 10000-65535 动态生成的话返回409则再次尝试
        let params = function::CreateTunnelParams {
            token: chml_api.get_token().unwrap().to_string(),
            tunnelname: "dyTi7y8a".to_string(),
            node: "中国香港".to_string(),
            localip: "127.0.0.1".to_string(),
            port_type: "TCP".to_string(),
            local_port: 8999,
            encryption: false,
            compression: false,
            extra_params: "".to_string(),
            remote_port: 64272,
        };

        let res = chml_api.create_tunnel(&params).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_delete_tunnel() {
        init_logger();
        let chml_api = ChmlApi::from_env().unwrap();

        let res = chml_api.delete_tunnel("239167").await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_update_tunnel() {
        init_logger();
        let chml_api = ChmlApi::from_env().unwrap();

        let tunnel_update = schema::TunnelUpdate {
            tunnelid: 239172,
            tunnelname: "dddi7y8a".to_string(),
            node: "中国香港".to_string(),
            port_type: "tcp".to_string(),
            localport: 8999,
            encryption: false,
            compression: false,
            localip: "127.0.0.1".to_string(),
            remoteport: 64272,
        };

        let res = chml_api.update_tunnel(tunnel_update).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_tunnel_config() {
        init_logger();
        let chml_api = ChmlApi::from_env().unwrap();

        let res = chml_api.tunnel_config("北京多线4", &["RDP"]).await;
        println!("{:?}", res);
    }
}
