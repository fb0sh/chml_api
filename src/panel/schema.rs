use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PanelInfo {
    pub tunnel_amount: u64,
    pub node_amount: u64,
    pub user_amount: u64,
    pub friend_links: Vec<FriendLink>,
}

#[derive(Debug, Deserialize)]
pub struct FriendLink {
    pub name: String,
    pub description: Option<String>, // JSON 中可能为 null
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerMetrics {
    pub metrics: Metrics,
    #[serde(rename = "serverName")]
    pub server_name: String,
    pub load: f64,
}

#[derive(Debug, Deserialize)]
pub struct Metrics {
    pub cpu: f64,
    pub memory: f64,
    pub steal: f64,
    #[serde(rename = "ioLatency")]
    pub io_latency: f64,
    #[serde(rename = "threadContention")]
    pub thread_contention: f64,
}
