pub mod prelude;
pub mod res;
pub mod user;

pub const BASE_URL: &str = "http://cf-v2.uapis.cn";

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
}

pub fn init_logger() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter("debug") // RUST_LOG=debug 可覆盖
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
