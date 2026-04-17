use super::{WechatClient, message::TypingTicket};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

impl WechatClient {
    pub async fn get_config(&self) -> crate::Result<TypingTicket> {
        let (user_id, context_token) = self.session_context_unwrap().await?;
        let text = self
            .create_post_request(
                "/ilink/bot/getconfig",
                &json!({
                    "ilink_user_id": &user_id,
                    "context_token": &context_token,
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
        let resp = serde_json::from_str::<GetConfigResponse>(&text)?;
        match resp {
            GetConfigResponse::Ok { typing_ticket, .. } => Ok(typing_ticket),
            GetConfigResponse::Err { ret, errmsg } => Err(anyhow!(
                "getconfig failed, got unexpected code: {ret}, err: {errmsg:?}"
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum GetConfigResponse {
    Ok {
        ret: i32,
        typing_ticket: TypingTicket,
    },
    Err {
        ret: i32,
        errmsg: Option<String>,
    },
}
