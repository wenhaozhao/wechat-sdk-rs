use derive_more::{Deref, Display, From, FromStr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatAccount {
    pub account_id: WechatAccountId,
    pub bot_id: String,
    pub bot_token: String,
    pub base_url: String,
}

#[derive(Debug, Clone, From, FromStr, Deref, Serialize, Deserialize, Display)]
pub struct WechatAccountId(String);

#[derive(
    Debug,
    Clone,
    Deref,
    From,
    FromStr,
    Serialize,
    Deserialize,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
)]
pub struct WechatUserId(String);

mod auth;
mod qr_code_api;
pub use qr_code_api::QrCodeUrl;
#[cfg(test)]
mod tests {
    use crate::account::WechatAccount;
    use crate::client::WechatConfig;

    #[tokio::test]
    async fn test_from_qr_auth() -> crate::Result<()> {
        let config = WechatConfig::from_env()?;
        let http_client = reqwest::Client::default();
        let account = WechatAccount::auth(&config, &http_client, async |url| {
            println!("Open url: {} and scan qr_code for login", url);
            Ok(())
        })
        .await?;
        let json = serde_json::to_string_pretty(&account)?;
        println!("{json}");
        Ok(())
    }
}
