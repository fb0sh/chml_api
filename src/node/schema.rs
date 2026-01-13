use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct Node {
    pub id: u64,
    pub name: String,
    pub area: String,
    pub nodegroup: String,
    #[serde(deserialize_with = "yes_no_to_bool")]
    pub china: bool,
    #[serde(deserialize_with = "yes_no_to_bool")]
    pub web: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub udp: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub fangyu: bool,
    pub notes: String,
}

#[derive(Debug, Deserialize)]
pub struct NodeInfo {
    pub id: u64,
    pub name: String,
    pub area: String,
    pub nodegroup: String,
    pub state: String,
    pub port: u16,
    pub adminPort: u16,
    pub real_IP: String,
    pub realIp: String,
    pub ip: String,
    pub auth: String,
    pub apitoken: String,
    pub nodetoken: String,
    pub version: String,
    pub coordinates: String,
    pub rport: String,
    pub totalTrafficIn: u64,
    pub totalTrafficOut: u64,
    pub bandwidth_usage_percent: u8,
    pub load1: f64,
    pub load5: f64,
    pub load15: f64,
    #[serde(deserialize_with = "yes_no_to_bool")]
    pub china: bool,
    #[serde(deserialize_with = "yes_no_to_bool")]
    pub web: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub udp: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub fangyu: bool,
    pub toowhite: bool,
    pub notes: String,

    // 可选字段
    pub cpu_info: Option<String>,
    pub storage_total: Option<u64>,
    pub storage_used: Option<u64>,
    pub memory_total: Option<u64>,
    pub num_cores: Option<u8>,
    pub uptime_seconds: Option<u64>,
    pub ipv6: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeStats {
    pub id: u64,
    pub node_name: String,
    pub nodegroup: String,
    pub state: String,
    pub total_traffic_in: u64,
    pub total_traffic_out: u64,
    pub bandwidth_usage_percent: u8,
    pub current_upload_usage_percent: u8,
    pub cpu_usage: f64,
    pub cur_counts: u64,
    pub client_counts: u64,
    pub tunnel_counts: u64,
}
#[derive(Debug, Deserialize)]
pub struct NodeUptime {
    pub node_name: String,
    pub state: String,
    pub id: u64,
    pub group: String,
    pub history_uptime: Vec<UptimeRecord>,
}

#[derive(Debug, Deserialize)]
pub struct UptimeRecord {
    pub recorded_at: String, // ISO 日期，例如 "2026-01-13"
    pub uptime: f64,         // 百分比，例如 100.0
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeDetails {
    pub total_traffic_in: u64,
    pub cpu_info: Option<String>,
    pub num_cores: Option<u32>,
    pub coordinates: String,
    pub storage_total: Option<u64>,
    pub load5: f64,
    pub version: String,
    pub load1: f64,
    pub total_traffic_out: u64,
    pub uptime_seconds: Option<u64>,
    pub memory_total: Option<u64>,
    pub storage_used: Option<u64>,
    pub load15: f64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NodeStatus {
    pub proxy_https: u32,
    pub download_bandwidth_usage_percent: u32,
    pub cur_conns: u32,
    pub sent_packets: u64,
    pub memory_used: u64,
    pub active_conn: u32,
    pub recv_packets: u64,
    pub proxy_tcp: u32,
    pub proxy_udp: u32,
    pub proxy_http: u32,
    pub upload_bandwidth_usage_percent: u32,
    pub cpu_usage: f64,
    pub page_tables: u64,
    pub passive_conn: u32,
    pub timestamp: String, // 或者用 chrono::DateTime
    pub client_counts: u32,
}
// "yes"/"no" -> bool
fn yes_no_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.eq_ignore_ascii_case("yes"))
}

// "true"/"false" -> bool
fn str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.eq_ignore_ascii_case("true"))
}
