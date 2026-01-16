use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tunnel {
    pub id: Option<u64>, // 可能为 null
    pub name: String,
    pub localip: String,
    #[serde(rename = "type")]
    pub r#type: String, // 避免关键字冲突
    pub nport: u16,
    pub dorp: String,
    pub node: String,
    #[serde(deserialize_with = "str_to_bool")]
    pub state: bool, // "true"/"false" -> bool
    pub userid: u64,
    #[serde(deserialize_with = "str_to_bool")]
    pub encryption: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub compression: bool,
    pub ap: String,
    pub uptime: Option<String>, // 可能为 null
    pub client_version: Option<String>,
    pub today_traffic_in: Option<u64>,
    pub today_traffic_out: Option<u64>,
    pub cur_conns: Option<u32>,
    pub nodestate: Option<String>,
    pub ip: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TunnelUpdate {
    pub tunnelid: u64,
    pub tunnelname: String,
    pub node: String,
    #[serde(rename = "porttype")]
    pub port_type: String,
    pub localport: u16,
    #[serde(deserialize_with = "str_to_bool")]
    pub encryption: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub compression: bool,
    pub localip: String,
    pub remoteport: u16,
}
// 自定义反序列化字符串 "true"/"false" -> bool
fn str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s == "true")
}
