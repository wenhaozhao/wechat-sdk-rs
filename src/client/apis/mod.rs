use crate::WECHAT_SDK_VERSION;
use crate::client::WechatClient;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reqwest::RequestBuilder;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

mod get_updates;
pub mod message;

mod send_message;

mod get_config;
mod send_typing;
mod cdn;

impl WechatClient {
    fn create_post_request<Body>(
        &self,
        endpoint: &str,
        body: &Body,
    ) -> crate::Result<RequestBuilder>
    where
        Body: Serialize,
    {
        let body = PostReqBody::from(body);
        let account = self.account()?;
        let url = &format!("{}{}", &account.base_url, endpoint);
        let builder = self
            .http_client
            .post(url)
            .header("Content-Type", "application/json")
            .header("AuthorizationType", "ilink_bot_token")
            .header("X-WECHAT-UIN", random_wechat_uin())
            .header("iLink-App-ClientVersion", 0)
            .bearer_auth(&account.bot_token)
            .json(&body);
        Ok(builder)
    }
}

#[derive(Debug, Serialize)]
struct PostReqBody<T>
where
    T: Serialize,
{
    #[serde(flatten)]
    body: T,
    base_info: PostReqBodyBaseInfo,
}

#[derive(Debug, Serialize)]
struct PostReqBodyBaseInfo {
    channel_version: String,
}

impl<T> From<T> for PostReqBody<T>
where
    T: Serialize,
{
    fn from(body: T) -> Self {
        Self {
            body,
            base_info: PostReqBodyBaseInfo {
                channel_version: WECHAT_SDK_VERSION.to_string(),
            },
        }
    }
}

fn random_wechat_uin() -> String {
    let now_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let reduced = (now_nanos % u32::MAX as u128) as u32;
    STANDARD.encode(reduced.to_string().as_bytes())
}
