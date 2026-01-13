use crate::domain::schema::{Domain, UserDomain};
use crate::{ChmlApi, res::ApiResult};

use crate::prelude::*;

/// list available domains
pub async fn get_available_domains(chml: &ChmlApi) -> ApiResult<Vec<Domain>> {
    let api_url = chml.endpoint("/list_available_domains");

    let req = chml.client.get(api_url).build()?;

    debug!("get available domains request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Vec<Domain>>>()
        .await?;
    debug!("get available domains response: {:?}", res);
    Ok(res)
}

/// get user free domains
pub async fn get_user_free_domains(chml: &ChmlApi) -> ApiResult<Vec<UserDomain>> {
    let api_url = chml.endpoint("/get_user_free_subdomains");

    let req = chml
        .client
        .get(api_url)
        .query(&[("token", chml.get_token()?)])
        .build()?;

    debug!("get user free domains request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Vec<UserDomain>>>()
        .await?;
    debug!("get user free domains response: {:?}", res);
    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDomainParams {
    pub domain: String,
    pub record: String,
    pub r#type: String,
    pub ttl: String,
    pub target: String,
    pub remarks: Option<String>,
}

/// create free subdomain
pub async fn create_free_subdomain(chml: &ChmlApi, params: &CreateDomainParams) -> ApiResult<()> {
    let api_url = chml.endpoint("/create_free_subdomain");

    let req = chml
        .client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", chml.get_token()?))
        .json(params)
        .build()?;

    debug!("create free subdomain request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("create free subdomain response: {:?}", res);
    Ok(res)
}

/// delete free subdomain
pub async fn delete_free_subdomain(chml: &ChmlApi, domain: &str, record: &str) -> ApiResult<()> {
    let api_url = chml.endpoint("/delete_free_subdomain");

    let req = chml
        .client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", chml.get_token()?))
        .json(&serde_json::json!({
            "domain": domain,
            "record": record
        }))
        .build()?;

    debug!("delete free subdomain request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("delete free subdomain response: {:?}", res);
    Ok(res)
}

/// update free subdomain
pub async fn update_free_subdomain(chml: &ChmlApi, params: &CreateDomainParams) -> ApiResult<()> {
    let api_url = chml.endpoint("/update_free_subdomain");

    let req = chml
        .client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", chml.get_token()?))
        .json(params)
        .build()?;

    debug!("update free subdomain request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("update free subdomain response: {:?}", res);
    Ok(res)
}
