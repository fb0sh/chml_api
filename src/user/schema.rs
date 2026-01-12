use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub password: Option<String>, // null -> None
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
    pub scgm: Option<String>, // null -> None
    pub regtime: String,
    pub realname_count: u32,
    pub total_download: u64,
    pub total_upload: u64,
    pub tunnelCount: u32,
    pub totalCurConns: u32,
}
// 为什么这么奇怪，风格都不统一呢？？？？ 奇怪的混合风格
// http 动词也很奇怪，登陆哪有用get的，而且还是明文传输，我有点想不明白，为什么要这样设计
