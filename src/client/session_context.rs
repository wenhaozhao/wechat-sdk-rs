use crate::account::WechatUserId;
use crate::client::WechatConfig;
use crate::client::message::ContextToken;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub(in crate::client) struct SessionContext {
    pub user_id: Option<WechatUserId>,
    pub context_token: Option<ContextToken>,
    pub get_updates_buf: Option<String>,
}

impl SessionContext {
    async fn path(config: &WechatConfig) -> crate::Result<PathBuf> {
        let session_ctx_path = config
            .account_state_path()
            .await?
            .join("session_context.json");
        Ok(session_ctx_path)
    }
    pub(in crate::client) async fn load(config: &WechatConfig) -> crate::Result<Self> {
        let path = Self::path(config).await?;
        let session_ctx = if path.exists() {
            tokio::fs::read_to_string(&path)
                .await
                .ok()
                .and_then(|json| serde_json::from_str::<SessionContext>(&json).ok())
                .unwrap_or_default()
        } else {
            Default::default()
        };
        Ok(session_ctx)
    }

    pub(in crate::client) async fn flush(&self, config: &WechatConfig) -> crate::Result<()> {
        let path = Self::path(config).await?;
        let json = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
}
