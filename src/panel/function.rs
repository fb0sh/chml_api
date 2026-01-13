use super::schema::PanelInfo;
use crate::{panel::schema::ServerMetrics, prelude::*};

pub async fn get_panel_info(chml: &ChmlApi) -> ApiResult<PanelInfo> {
    let api_url = chml.endpoint("/panelinfo");
    let req = chml.client.get(api_url).build()?;
    debug!("get panel info request: {:?}", req.url());
    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<PanelInfo>>()
        .await?;
    debug!("get panel info response: {:?}", res);
    Ok(res)
}

pub async fn get_api_server_status(chml: &ChmlApi) -> Result<ServerMetrics, ApiError> {
    let api_url = chml.endpoint("/api/server-status");
    let req = chml.client.get(api_url).build()?;
    debug!("get api server status request: {:?}", req.url());
    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ServerMetrics>()
        .await?;
    debug!("get api server status response: {:?}", res);
    Ok(res)
}
