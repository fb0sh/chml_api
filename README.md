# chml_api

Rust SDK for chml - 一个用于与 chml API 交互的 Rust 客户端库。

## 功能特性

- ✅ 用户登录与认证
- ✅ 用户注册
- ✅ 邮箱验证码发送
- ✅ 获取用户信息
- ✅ Token 刷新
- ✅ 每日签到
- ✅ 密码重置
- ✅ 用户信息更新（用户名、QQ、头像等）
- 📝 完整的日志追踪
- 🛡️ 类型安全的 API 响应处理

## 安装

将以下内容添加到你的 `Cargo.toml` 中：

```toml
[dependencies]
chml_api = "0.1.0"
```

## 快速开始

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

## 许可证

MIT

## 作者

fb0sh <fb0sh@outlook.com>

## 仓库

https://github.com/fb0sh/chml_api
