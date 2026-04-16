use super::message::{SessionContext, WechatMessage};
use crate::client::WechatClient;
use anyhow::anyhow;
use derive_more::with_trait::IntoIterator;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

impl WechatClient {
    pub async fn get_updates(&self) -> crate::Result<WechatMessages> {
        let SessionContext {
            context_token,
            get_updates_buf,
        } = SessionContext::load(&self.config).await?;
        let text = self
            .create_post_request(
                "/ilink/bot/getupdates",
                &json!({
                    "get_updates_buf": get_updates_buf.as_deref().unwrap_or_default(),
                }),
            )?
            .timeout(
                self.config
                    .qr_login_timeout
                    .unwrap_or(Duration::from_secs(120)),
            )
            .send()
            .await?
            .text()
            .await?;
        let resp = serde_json::from_str::<GetUpdatesResponse>(&text)?;
        match resp {
            GetUpdatesResponse::Ok {
                msgs,
                get_updates_buf,
            } => {
                let _ = SessionContext {
                    context_token: msgs
                        .iter()
                        .map(|it| it.context_token.clone())
                        .last()
                        .or(context_token),
                    get_updates_buf,
                }
                .flush(&self.config)
                .await?;
                Ok(msgs)
            }
            GetUpdatesResponse::Err { errcode, errmsg } => Err(anyhow!(
                "get_updates failed, got unexpected code: {errcode}, err: {errmsg}"
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum GetUpdatesResponse {
    Ok {
        msgs: WechatMessages,
        get_updates_buf: Option<String>,
    },
    Err {
        errcode: i32,
        errmsg: String,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, Deref, DerefMut, IntoIterator)]
pub struct WechatMessages(Vec<WechatMessage>);
