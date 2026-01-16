pub mod function;
pub mod schema;
use crate::ChmlApi;
use crate::res::ApiResult;

impl ChmlApi {
    /// get available domains
    pub async fn list_available_domains(&self) -> ApiResult<Vec<schema::Domain>> {
        function::get_available_domains(self).await
    }

    /// get user free domains
    pub async fn get_user_free_domains(&self) -> ApiResult<Vec<schema::UserDomain>> {
        function::get_user_free_domains(self).await
    }

    /// create free subdomain
    pub async fn create_free_subdomain(
        &self,
        params: &function::CreateDomainParams,
    ) -> ApiResult<()> {
        function::create_free_subdomain(self, params).await
    }

    /// delete free subdomain
    pub async fn delete_free_subdomain(&self, domain: &str, record: &str) -> ApiResult<()> {
        function::delete_free_subdomain(self, domain, record).await
    }

    /// update free subdomain
    pub async fn update_free_subdomain(
        &self,
        params: &function::CreateDomainParams,
    ) -> ApiResult<()> {
        function::update_free_subdomain(self, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn test_list_available_domains() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.list_available_domains().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_user_free_domains() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml.get_user_free_domains().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_create_free_subdomain() {
        let chml = ChmlApi::from_env().unwrap();
        let params = function::CreateDomainParams {
            domain: "yeshan.fun".to_string(),
            record: "c123c".to_string(),
            r#type: "A".to_string(),
            ttl: "10分钟".to_string(),
            target: "1.1.1.1".to_string(),
            remarks: Some("test".to_string()),
        };
        let res = chml.create_free_subdomain(&params).await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_delete_free_subdomain() {
        let chml = ChmlApi::from_env().unwrap();
        let res = chml
            .delete_free_subdomain("yeshan.fun", "c123c")
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_update_free_subdomain() {
        let chml = ChmlApi::from_env().unwrap();
        let params = function::CreateDomainParams {
            domain: "yeshan.fun".to_string(),
            record: "c123c".to_string(),
            r#type: "A".to_string(),
            ttl: "30分钟".to_string(),
            target: "1.1.1.1".to_string(),
            remarks: Some("test".to_string()),
        };
        let res = chml.update_free_subdomain(&params).await.unwrap();
        println!("{:?}", res);
    }
}
