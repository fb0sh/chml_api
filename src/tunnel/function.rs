use super::schema::Tunnel;
use crate::{prelude::*, tunnel::schema::TunnelUpdate};

/// get_tunnels
pub async fn get_tunnels(chml_api: &ChmlApi) -> ApiResult<Vec<Tunnel>> {
    let api_url = chml_api.endpoint("/tunnel");

    let req = chml_api
        .client
        .get(api_url)
        .query(&[("token", chml_api.get_token()?)])
        .build()?;
    debug!("get tunnels request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Vec<Tunnel>>>()
        .await?;
    debug!("get tunnels response: {:?}", res);

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTunnelParams {
    pub token: String,
    pub tunnelname: String,
    pub node: String,
    pub localip: String,
    #[serde(rename = "porttype")]
    pub port_type: String, // 避免字段命名风格不一致
    #[serde(rename = "localport")]
    pub local_port: u16,
    pub encryption: bool,
    pub compression: bool,
    #[serde(rename = "extraparams")]
    pub extra_params: String,
    #[serde(rename = "remoteport")]
    pub remote_port: u16,
}
/// create_tunnel
pub async fn create_tunnel(chml_api: &ChmlApi, params: &CreateTunnelParams) -> ApiResult<Tunnel> {
    let api_url = chml_api.endpoint("/create_tunnel");

    let req = chml_api.client.post(api_url).json(&params).build()?;
    debug!("create tunnel request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Tunnel>>()
        .await?;
    debug!("create tunnel response: {:?}", res);
    // 创建出的隧道 返回为啥 id 是null啊，
    Ok(res)
}

/// delete_tunnel
pub async fn delete_tunnel(chml_api: &ChmlApi, tunnel_id: &str) -> ApiResult<()> {
    let api_url = chml_api.endpoint("/delete_tunnel");

    let req = chml_api
        .client
        .post(api_url)
        .query(&[("token", chml_api.get_token()?), ("tunnelid", tunnel_id)])
        .build()?;
    debug!("delete tunnel request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<()>>()
        .await?;
    debug!("delete tunnel response: {:?}", res);

    Ok(res)
}

/// update_tunnel
pub async fn update_tunnel(chml_api: &ChmlApi, tunnel_update: TunnelUpdate) -> ApiResult<Tunnel> {
    let api_url = chml_api.endpoint("/update_tunnel");

    let req = chml_api
        .client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", chml_api.get_token()?))
        .json(&tunnel_update)
        .build()?;
    debug!("update tunnel config request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Tunnel>>()
        .await?;
    debug!("update tunnel config response: {:?}", res);

    Ok(res)
}

/// 这里怎么又用Bearer了， 能统一一下认证方式吗？
/// get_tunnel_config
pub async fn get_tunnel_config(
    chml_api: &ChmlApi,
    node: &str,
    tunnel_names: &[&str],
) -> ApiResult<String> {
    let api_url = chml_api.endpoint("/tunnel_config");

    let req = chml_api
        .client
        .get(api_url)
        .header("Authorization", format!("Bearer {}", chml_api.get_token()?))
        .query(&[
            ("node", node),
            ("tunnel_names", tunnel_names.join(",").as_str()),
        ])
        .build()?;
    debug!("get tunnel config request: {:?}", req.url());

    let res = chml_api
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<String>>()
        .await?;
    debug!("get tunnel config response: {:?}", res);

    Ok(res)
}
