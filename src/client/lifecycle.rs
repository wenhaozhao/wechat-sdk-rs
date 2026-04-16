use crate::account::{QrCodeUrl, WechatAccount};
use crate::client::{WechatClient, WechatConfig};
use std::time::Duration;

impl WechatClient {
    pub async fn new<C: Into<WechatConfig>>(config: C) -> crate::Result<Self> {
        let config = config.into();
        let http_client = reqwest::Client::builder()
            .timeout(config.http_timeout.unwrap_or(Duration::from_secs(2)))
            .build()?;
        Ok(Self {
            config,
            http_client,
            account: Default::default(),
        })
    }
}

impl WechatClient {
    pub async fn init<F, Fut>(mut self, qr_auth_handle: F) -> crate::Result<Self>
    where
        Fut: Future<Output = crate::Result<()>>,
        F: Fn(QrCodeUrl) -> Fut,
    {
        if self.account.is_some() {
            return Ok(self);
        }
        let account = WechatAccount::auth(&self.config, &self.http_client, qr_auth_handle).await?;
        self.account.replace(account);
        Ok(self)
    }

    pub async fn start(&self) -> crate::Result<()> {
        Ok(())
    }
}
