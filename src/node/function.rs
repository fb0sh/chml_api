use crate::{
    node::schema::{Node, NodeDetails, NodeInfo, NodeStats, NodeStatus, NodeUptime},
    prelude::*,
};

/// 获取节点列表
pub async fn get_node_list(chml: &ChmlApi) -> ApiResult<Vec<Node>> {
    let api_url = chml.endpoint("/node");

    let req = chml.client.get(api_url).build()?;
    debug!("get node list request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Vec<Node>>>()
        .await?;
    debug!("get node list response: {:?}", res);
    Ok(res)
}

/// 获取详情
pub async fn get_node_info(chml: &ChmlApi, node_name: &str) -> ApiResult<NodeInfo> {
    let api_url = chml.endpoint("/nodeinfo");

    let req = chml
        .client
        .get(api_url)
        .query(&[("token", chml.get_token()?), ("node", node_name)])
        .build()?;
    debug!("get node info request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<NodeInfo>>()
        .await?;
    debug!("get node info response: {:?}", res);
    Ok(res)
}

/// node stats
pub async fn get_node_stats(chml: &ChmlApi) -> ApiResult<Vec<NodeStats>> {
    let api_url = chml.endpoint("/node_stats");

    let req = chml.client.get(api_url).build()?;
    debug!("get node status request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Vec<NodeStats>>>()
        .await?;
    debug!("get node status response: {:?}", res);
    Ok(res)
}

/// node uptime
pub async fn get_node_uptime(
    chml: &ChmlApi,
    node_name: &str,
    days: i32,
) -> ApiResult<Vec<NodeUptime>> {
    let api_url = chml.endpoint("/node_uptime");

    let req = chml
        .client
        .get(api_url)
        .query(&[("node", node_name), ("time", &days.to_string())])
        .build()?;
    debug!("get node uptime request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<Vec<NodeUptime>>>()
        .await?;
    debug!("get node uptime response: {:?}", res);
    Ok(res)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeStatusResult {
    pub node_name: String,
    pub node_details: NodeDetails,
    pub status_list: Vec<NodeStatus>,
}

pub async fn get_node_status_info(chml: &ChmlApi, node_name: &str) -> ApiResult<NodeStatusResult> {
    let api_url = chml.endpoint("/node_status_info");

    let req = chml
        .client
        .get(api_url)
        .query(&[("nodename", node_name)])
        .build()?;
    debug!("get node status request: {:?}", req.url());

    let res = chml
        .client
        .execute(req)
        .await?
        .json::<ApiResponse<NodeStatusResult>>()
        .await?;
    debug!("get node status response: {:?}", res);
    Ok(res)
}
