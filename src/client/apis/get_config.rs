use crate::client::WechatClient;
use crate::client::message::{ContextToken, SessionContext, ToUserId, TypingTicket};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

impl WechatClient {
    pub async fn get_config<TU>(
        &self,
        to_user_id: TU,
        context_token: Option<&ContextToken>,
    ) -> crate::Result<TypingTicket>
    where
        TU: Into<ToUserId>,
    {
        let to_user_id = to_user_id.into();
        let context_token = if let Some(context_token) = context_token {
            Some(context_token.clone())
        } else {
            let SessionContext { context_token, .. } = SessionContext::load(&self.config).await?;
            context_token
        };
        let Some(context_token) = context_token else {
            return Err(anyhow!(
                "context_token is required, please init it by recv message first!!!"
            ));
        };
        let text = self
            .create_post_request(
                "/ilink/bot/getconfig",
                &json!({
                    "ilink_user_id": &to_user_id,
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
