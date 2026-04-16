//!
//! Wechat SDK for robot
//!
//!
pub mod account;
pub mod client;

pub const WECHAT_SDK_NAME: &str = env!("CARGO_PKG_NAME");
pub const WECHAT_SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

pub const WECHAT_BASE_URL: &str = "https://ilinkai.weixin.qq.com";
