use crate::account::{QrCodeUrl, WechatAccount};
use crate::client::WechatClient;

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
