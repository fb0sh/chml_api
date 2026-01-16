use crate::res::ApiError;
// utils
pub mod prelude;
pub mod res;
// modules
pub mod domain;
pub mod node;
pub mod panel;
pub mod tunnel;
pub mod user;
pub mod util;

use dotenvy::dotenv;
use std::env;

/// 如果token能统一位置就好了，可以更抽象和简化，一般都用 Authorization: Bearer <token> 这种形式
pub struct ChmlApi {
    pub base_url: String,
    pub token: Option<String>,
    pub client: reqwest::Client,
}

impl ChmlApi {
    pub fn new(base_url: &str) -> Self {
        ChmlApi {
            base_url: base_url.to_string(),
            token: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenv().ok();
        let base_url = env::var("CHML_API_BASE_URL")?;
        let token = env::var("CHML_API_TOKEN")?;
        Ok(Self {
            base_url,
            token: Some(token),
            client: reqwest::Client::new(),
        })
    }

    pub fn new_with_token(base_url: &str, token: &str) -> Self {
        ChmlApi {
            base_url: base_url.to_string(),
            token: Some(token.to_string()),
            client: reqwest::Client::new(),
        }
    }

    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }

    pub fn get_token(&self) -> Result<&str, ApiError> {
        self.token.as_deref().ok_or(ApiError::NoToken)
    }
}

pub mod schema {
    pub use super::domain::schema::{Domain, UserDomain};
    pub use super::node::schema::{Node, NodeDetails, NodeInfo, NodeStats, NodeStatus, NodeUptime};
    pub use super::panel::schema::{FriendLink, Metrics, PanelInfo, ServerMetrics};
    pub use super::tunnel::schema::{Tunnel, TunnelUpdate};
    pub use super::user::schema::UserInfo;
    pub use super::util::schema::{Message, UserLog};
}

pub fn init_logger() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter("debug") // RUST_LOG=debug 可覆盖
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
