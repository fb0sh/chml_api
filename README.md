# chml_api

[![Crates.io](https://img.shields.io/crates/v/chml_api)](https://crates.io/crates/chml_api)
[![Documentation](https://docs.rs/chml_api/badge.svg)](https://docs.rs/chml_api)
[![License](https://img.shields.io/crates/l/chml_api)](https://github.com/fb0sh/chml_api)

Rust SDK for chml - 一个用于与 chml API 交互的 Rust 客户端库。

## 功能特性

### 用户管理
- ✅ 用户登录与认证
- ✅ 用户注册
- ✅ 邮箱验证码发送
- ✅ 获取用户信息
- ✅ Token 刷新
- ✅ 每日签到
- ✅ 密码重置
- ✅ 用户信息更新（用户名、QQ、头像等）

### 隧道管理
- ✅ 获取隧道列表
- ✅ 创建隧道
- ✅ 删除隧道
- ✅ 更新隧道配置
- ✅ 获取隧道配置文件

### 面板管理
- ✅ 获取面板信息（隧道数、节点数、用户数等）
- ✅ 获取服务器状态（CPU、内存、负载等指标）

### 其他
- 📝 完整的日志追踪
- 🛡️ 类型安全的 API 响应处理
- 🔌 支持环境变量配置

## 安装

将以下内容添加到你的 `Cargo.toml` 中：

```toml
[dependencies]
chml_api = "0.1.2"
```

## 快速开始

### 使用环境变量（推荐）

创建 `.env` 文件：

```env
CHML_API_BASE_URL=http://cf-v2.uapis.cn
CHML_API_TOKEN=your_token_here
```

```rust
use chml_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量加载配置
    let api = ChmlApi::from_env()?;

    // 获取用户信息
    let user_info = api.user_info().await?.into_result()?;
    println!("用户名: {}", user_info.username);

    // 获取隧道列表
    let tunnels = api.tunnel().await?.into_result()?;
    println!("隧道数量: {}", tunnels.len());

    // 获取面板信息
    let panel_info = api.panelinfo().await?.into_result()?;
    println!("总隧道数: {}", panel_info.tunnel_amount);
    println!("总节点数: {}", panel_info.node_amount);

    // 获取服务器状态
    let server_status = api.server_status().await?;
    println!("服务器: {}", server_status.server_name);
    println!("CPU: {}%", server_status.metrics.cpu);

    Ok(())
}
```

### 基本使用

```rust
use chml_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志（可选）
    init_logger();

    // 创建 API 客户端
    let mut api = ChmlApi::new(BASE_URL);

    // 登录
    let login_params = LoginParams {
        username: "your_username".to_string(),
        password: "your_password".to_string(),
    };
    let user_info = api.login(&login_params).await?;
    println!("登录成功，用户: {}", user_info.username);

    // 获取用户信息
    let user_info = api.user_info().await?.into_result()?;
    println!("用户积分: {}", user_info.integral);

    Ok(())
}
```

### 使用已有 Token

```rust
use chml_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用已有 token 创建客户端
    let api = ChmlApi::new_with_token(BASE_URL, "your_token_here");

    // 直接调用需要认证的接口
    let user_info = api.user_info().await?.into_result()?;
    println!("用户名: {}", user_info.username);

    Ok(())
}
```

## API 文档

### 用户认证

#### 登录

```rust
let mut api = ChmlApi::new(BASE_URL);
let login_params = LoginParams {
    username: "username".to_string(),
    password: "password".to_string(),
};
let user_info = api.login(&login_params).await?;
```

#### 注册

```rust
let api = ChmlApi::new(BASE_URL);
let register_params = RegisterParams {
    username: "new_user".to_string(),
    password: "password123".to_string(),
    mail: "user@example.com".to_string(),
    qq: 123456789,
    code: 123456,
};
api.register(&register_params).await?;
```

#### 发送邮箱验证码

```rust
let api = ChmlApi::new(BASE_URL);
let email_params = SendEmailCodeParams {
    r#type: "1".to_string(),  // 1: 注册, 2: 重置 token
    mail: "user@example.com".to_string(),
    lot_number: "lot_number".to_string(),
    captcha_output: "captcha".to_string(),
    pass_token: vec!["token".to_string()],
    gen_time: "timestamp".to_string(),
};
api.send_email_code(&email_params).await?;
```

### 用户信息

#### 获取用户信息

```rust
let user_info = api.user_info().await?.into_result()?;
println!("用户名: {}", user_info.username);
println!("积分: {}", user_info.integral);
println!("带宽: {}", user_info.bandwidth);
```

#### 更新用户名

```rust
api.update_username("new_username").await?;
```

#### 更新 QQ

```rust
api.update_qq("123456789").await?;
```

#### 更新头像

```rust
api.update_userimg("https://example.com/avatar.jpg").await?;
```

### 隧道管理

#### 获取隧道列表

```rust
let tunnels = api.tunnel().await?.into_result()?;
for tunnel in tunnels {
    println!("隧道名称: {}, 状态: {}", tunnel.name, tunnel.state);
}
```

#### 创建隧道

```rust
let params = CreateTunnelParams {
    token: api.get_token()?.to_string(),
    tunnelname: "my_tunnel".to_string(),
    node: "中国香港".to_string(),
    localip: "127.0.0.1".to_string(),
    port_type: "TCP".to_string(),
    local_port: 8080,
    encryption: false,
    compression: false,
    extra_params: "".to_string(),
    remote_port: 12345,
};
let tunnel = api.create_tunnel(&params).await?.into_result()?;
println!("创建的隧道 ID: {:?}", tunnel.id);
```

#### 更新隧道

```rust
let tunnel_update = TunnelUpdate {
    tunnelid: 123456,
    tunnelname: "updated_tunnel".to_string(),
    node: "中国香港".to_string(),
    port_type: "tcp".to_string(),
    localport: 8080,
    encryption: false,
    compression: false,
    localip: "127.0.0.1".to_string(),
    remoteport: 12345,
};
let tunnel = api.update_tunnel(tunnel_update).await?.into_result()?;
```

#### 删除隧道

```rust
api.delete_tunnel("123456").await?;
```

#### 获取隧道配置文件

```rust
let config = api.tunnel_config("中国香港", &["tunnel1", "tunnel2"]).await?.into_result()?;
println!("配置文件:\n{}", config);
```

### 面板管理

#### 获取面板信息

```rust
let panel_info = api.panelinfo().await?.into_result()?;
println!("隧道数量: {}", panel_info.tunnel_amount);
println!("节点数量: {}", panel_info.node_amount);
println!("用户数量: {}", panel_info.user_amount);

for link in panel_info.friend_links {
    println!("友情链接: {} - {}", link.name, link.url);
}
```

#### 获取服务器状态

```rust
let server_status = api.server_status().await?;
println!("服务器名称: {}", server_status.server_name);
println!("负载: {}", server_status.load);
println!("CPU 使用率: {}%", server_status.metrics.cpu);
println!("内存使用率: {}%", server_status.metrics.memory);
println!("IO 延迟: {}", server_status.metrics.io_latency);
```

### 其他功能

#### 每日签到

```rust
let checkin_params = CheckinParams {
    // 签到参数
};
api.qiandao(&checkin_params).await?;
```

#### 重置 Token

```rust
let reset_params = ResetTokenParams {
    // 重置参数
};
api.retoken(&reset_params).await?;
```

#### 重置密码

```rust
let reset_password_params = ResetPasswordParams {
    // 重置密码参数
};
api.reset_password(&reset_password_params).await?;
```

## 数据结构

### UserInfo

```rust
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub password: Option<String>,
    pub userimg: String,
    pub qq: String,
    pub email: String,
    pub usertoken: String,
    pub usergroup: String,
    pub bandwidth: u32,
    pub tunnel: u32,
    pub realname: String,
    pub integral: u32,
    pub term: String,
    pub scgm: Option<String>,
    pub regtime: String,
    pub realname_count: u32,
    pub total_download: u64,
    pub total_upload: u64,
    pub tunnelCount: u32,
    pub totalCurConns: u32,
}
```

### Tunnel

```rust
pub struct Tunnel {
    pub id: Option<u64>,
    pub name: String,
    pub localip: String,
    pub r#type: String,  // "tcp" 或 "udp"
    pub nport: u16,      // 本地端口
    pub dorp: String,    // 远程端口（字符串形式）
    pub state: bool,     // 隧道状态
    pub userid: u64,
    pub encryption: bool,
    pub compression: bool,
    pub ap: String,
    pub uptime: Option<String>,
    pub client_version: Option<String>,
    pub today_traffic_in: Option<u64>,
    pub today_traffic_out: Option<u64>,
    pub cur_conns: Option<u32>,
    pub nodestate: Option<String>,
    pub ip: Option<String>,
}
```

### TunnelUpdate

```rust
pub struct TunnelUpdate {
    pub tunnelid: u64,
    pub tunnelname: String,
    pub node: String,
    pub port_type: String,
    pub localport: u16,
    pub encryption: bool,
    pub compression: bool,
    pub localip: String,
    pub remoteport: u16,
}
```

### PanelInfo

```rust
pub struct PanelInfo {
    pub tunnel_amount: u64,           // 隧道总数
    pub node_amount: u64,             // 节点总数
    pub user_amount: u64,             // 用户总数
    pub friend_links: Vec<FriendLink>, // 友情链接
}

pub struct FriendLink {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
}
```

### ServerMetrics

```rust
pub struct ServerMetrics {
    pub metrics: Metrics,
    pub server_name: String,  // 服务器名称
    pub load: f64,            // 负载
}

pub struct Metrics {
    pub cpu: f64,                    // CPU 使用率
    pub memory: f64,                 // 内存使用率
    pub steal: f64,                  // CPU 窃取时间
    pub io_latency: f64,             // IO 延迟
    pub thread_contention: f64,      // 线程竞争
}
```

### ApiResponse

```rust
pub struct ApiResponse<T> {
    pub msg: String,
    pub code: u16,
    pub state: String,
    pub data: Option<T>,
}
```

## 错误处理

```rust
use chml_api::res::ApiError;

match api.user_info().await {
    Ok(response) => {
        match response.into_result() {
            Ok(user_info) => println!("用户信息: {:?}", user_info),
            Err(ApiError::NoToken) => eprintln!("未设置 token"),
            Err(ApiError::Api { code, state, msg }) => {
                eprintln!("API 错误: code={}, state={}, msg={}", code, state, msg);
            }
            Err(e) => eprintln!("其他错误: {}", e),
        }
    }
    Err(e) => eprintln!("请求失败: {}", e),
}
```

## 日志配置

```rust
// 初始化日志，默认级别为 debug
init_logger();

// 也可以通过环境变量设置日志级别
// RUST_LOG=info cargo run
```

## 依赖项

- `tokio` - 异步运行时
- `serde` / `serde_json` - 序列化/反序列化
- `reqwest` - HTTP 客户端
- `thiserror` - 错误处理
- `tracing` / `tracing-subscriber` - 日志追踪
- `dotenvy` - 环境变量加载

## 环境变量

支持通过环境变量配置 API 客户端：

- `CHML_API_BASE_URL` - API 基础 URL（默认：`http://cf-v2.uapis.cn`）
- `CHML_API_TOKEN` - 认证 Token

## 许可证

MIT

## 作者

fb0sh <fb0sh@outlook.com>

## 仓库

https://github.com/fb0sh/chml_api

## Crates.io

https://crates.io/crates/chml_api
