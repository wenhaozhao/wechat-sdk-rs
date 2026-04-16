use std::path::PathBuf;
use std::time::Duration;

use crate::account::WechatAccount;
use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatConfig {
    pub state_path: PathBuf,
    pub account_id: super::account::WechatAccountId,
    pub http_timeout: Option<Duration>,
    pub qr_login_timeout: Option<Duration>,
    pub http_api_get_updates_timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct WechatClient {
    pub config: WechatConfig,
    pub http_client: Client,
    pub account: Option<WechatAccount>,
}
impl WechatClient {
    pub fn account(&self) -> crate::Result<&WechatAccount> {
        Ok(self.account.as_ref().ok_or(anyhow!("call init before"))?)
    }
}

mod config;
mod lifecycle;

mod apis;
pub use apis::*;
