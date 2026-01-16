use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLog {
    pub address: String,
    pub user_id: u64,
    pub extra_data: String, // 如果需要解析里面的 JSON 可以额外用 serde_json::Value
    pub action: String,
    pub id: u64,
    pub ip_address: String,
    pub category: String,
    pub resource_name: String,
    pub user_agent: String,
    pub status: String,
    pub timestamp: String, // 或者 chrono::DateTime<FixedOffset>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub publishTime: String, // 发布时间，ISO 8601 格式
    pub createdAt: String,   // 创建时间，ISO 8601 格式
    pub id: u64,             // 消息 ID
    pub title: String,       // 消息标题
    pub priority: u8,        // 消息优先级
    pub user: bool,          // 是否是用户消息
}
