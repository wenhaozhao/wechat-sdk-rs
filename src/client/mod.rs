use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::account::{WechatAccount, WechatAccountId, WechatUserId};
use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatConfig {
    pub state_path: PathBuf,
    pub account_id: WechatAccountId,
    pub http_timeout: Option<Duration>,
    pub qr_login_timeout: Option<Duration>,
    pub http_api_get_updates_timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct WechatClient {
    pub config: WechatConfig,
    pub http_client: Client,
    account: Option<WechatAccount>,
    session_context: Arc<RwLock<SessionContext>>,
}
impl WechatClient {
    pub async fn new<C: Into<WechatConfig>>(config: C) -> crate::Result<Self> {
        let config = config.into();
        let http_client = Client::builder()
            .timeout(config.http_timeout.unwrap_or(Duration::from_secs(2)))
            .build()?;
        let session_context = SessionContext::load(&config).await?;
        Ok(Self {
            config,
            http_client,
            account: Default::default(),
            session_context: Arc::new(RwLock::new(session_context)),
        })
    }

    fn account(&self) -> crate::Result<&WechatAccount> {
        Ok(self.account.as_ref().ok_or(anyhow!("call init before"))?)
    }

    async fn session_context(&self) -> crate::Result<SessionContext> {
        let ctx = {
            let ctx = self.session_context.read().await;
            ctx.clone()
        };
        Ok(ctx)
    }

    async fn session_context_unwrap(&self) -> crate::Result<(WechatUserId, ContextToken)> {
        let ctx = self.session_context().await?;
        if let (Some(user_id), Some(context_token)) = (ctx.user_id, ctx.context_token) {
            Ok((user_id, context_token))
        } else {
            Err(anyhow!(
                "the session context has not been initialized yet, please init it by recv message first!!!"
            ))
        }
    }
}

mod config;
mod lifecycle;

mod apis;
mod session_context;
use crate::client::message::ContextToken;
pub use apis::*;
use session_context::SessionContext;
