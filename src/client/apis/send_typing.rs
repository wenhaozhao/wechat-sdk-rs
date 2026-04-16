use crate::client::WechatClient;
use crate::client::apis::message::TypingTicket;
use crate::client::message::ToUserId;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::time::Duration;

impl WechatClient {
    pub async fn send_typing<TU>(
        &self,
        to_user_id: TU,
        typing_ticket: &TypingTicket,
    ) -> crate::Result<()>
    where
        TU: Into<ToUserId>,
    {
        self.send_typing_actual(to_user_id, typing_ticket, SendTypingStatus::Typing)
            .await
    }

    pub async fn send_typing_cannel<TU>(
        &self,
        to_user_id: TU,
        typing_ticket: &TypingTicket,
    ) -> crate::Result<()>
    where
        TU: Into<ToUserId>,
    {
        self.send_typing_actual(to_user_id, typing_ticket, SendTypingStatus::Cancel)
            .await
    }

    async fn send_typing_actual<TU>(
        &self,
        to_user_id: TU,
        typing_ticket: &TypingTicket,
        send_typing_status: SendTypingStatus,
    ) -> crate::Result<()>
    where
        TU: Into<ToUserId>,
    {
        let to_user_id = to_user_id.into();
        let text = self
            .create_post_request(
                "/ilink/bot/sendtyping",
                &json!({
                    "ilink_user_id": &to_user_id,
                    "typing_ticket": typing_ticket,
                    "status": send_typing_status,
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
        let resp = serde_json::from_str::<SendTypingResponse>(&text)?;
        match resp {
            SendTypingResponse::Ok { .. } => Ok(()),
            SendTypingResponse::Err { ret, errmsg } => Err(anyhow!(
                "get_updates failed, got unexpected code: {ret}, err: {errmsg}"
            )),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum SendTypingStatus {
    Typing = 1,
    Cancel = 2,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SendTypingResponse {
    Ok { ret: i32 },
    Err { ret: i32, errmsg: String },
}
