use super::{
    WechatClient,
    message::{MessageItems, MessageType},
};
use serde_json::json;
use std::time::Duration;

impl WechatClient {
    pub async fn send_message<MsgItems>(&self, msg_items: MsgItems) -> crate::Result<()>
    where
        MsgItems: Into<MessageItems>,
    {
        let (user_id, context_token) = self.session_context_unwrap().await?;
        let _resp = self
            .create_post_request(
                "/ilink/bot/sendmessage",
                &json!({
                    "msg": {
                        "client_id": uuid::Uuid::new_v4().to_string(),
                        "context_token": &context_token,
                        "from_user_id": "",
                        "to_user_id": &user_id,
                        "message_type": MessageType::Bot,
                        "message_state": 2, // FINISH
                        "item_list": &msg_items.into(),
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
