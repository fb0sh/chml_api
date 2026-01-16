use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: u64,
    pub domain: String,
    pub remarks: Option<String>,
    pub icpFiling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDomain {
    pub id: u64,
    pub userid: u64,
    pub domain: String,
    pub record: String,
    pub r#type: String,
    pub target: String,
    pub remarks: Option<String>,
    pub ttl: String,
}
