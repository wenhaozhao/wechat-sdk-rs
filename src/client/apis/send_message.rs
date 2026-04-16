use crate::client::WechatClient;
use crate::client::message::{ContextToken, MessageItems, MessageType, SessionContext, ToUserId};
use anyhow::anyhow;
use serde_json::json;
use std::time::Duration;

impl WechatClient {
    pub async fn send_message<MsgItems, TU>(
        &self,
        to_user_id: TU,
        msg_items: MsgItems,
        context_token: Option<ContextToken>,
    ) -> crate::Result<()>
    where
        MsgItems: Into<MessageItems>,
        TU: Into<ToUserId>,
    {
        let to_user_id = to_user_id.into();
        let msg_items = msg_items.into();
        let context_token = if let Some(context_token) = context_token {
            Some(context_token)
        } else {
            let SessionContext { context_token, .. } = SessionContext::load(&self.config).await?;
            context_token
        };
        let Some(context_token) = context_token else {
            return Err(anyhow!(
                "context_token is required, please init it by recv message first!!!"
            ));
        };

        let _resp = self
            .create_post_request(
                "/ilink/bot/sendmessage",
                &json!({
                    "msg": {
                        "client_id": uuid::Uuid::new_v4().to_string(),
                        "context_token": &context_token,
                        "from_user_id": "",
                        "to_user_id": &to_user_id,
                        "message_type": MessageType::Bot,
                        "message_state": 2, // FINISH
                        "item_list": &msg_items,
                    },
                }),
            )?
            .timeout(
                self.config
                    .qr_login_timeout
                    .unwrap_or(Duration::from_secs(120)),
            )
            .send()
            .await?;
        Ok(())
    }
}
