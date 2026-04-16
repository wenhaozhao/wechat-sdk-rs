use crate::WECHAT_BASE_URL;
use crate::account::WechatAccount;
use crate::client::WechatConfig;
use anyhow::anyhow;
use log::{info, warn};
use std::time::Duration;
use crate::account::qr_code_api::{fetch_qr_code, fetch_qr_code_status, ILinkBot, QrCodeResponse, QrCodeStatusResponse, QrCodeUrl, QrStatus};

impl WechatAccount {
    pub(crate) async fn auth<F, Fut>(
        config @ WechatConfig {
            account_id,
            qr_login_timeout,
            ..
        }: &WechatConfig,
        http_client: &reqwest::Client,
        qr_url_handler: F,
    ) -> crate::Result<Self>
    where
        Fut: Future<Output = crate::Result<()>>,
        F: Fn(QrCodeUrl) -> Fut,
    {
        if let Some(self_) = Self::from_account_state(&config).await? {
            return Ok(self_);
        }
        let QrCodeResponse { qrcode, qrcode_url } = fetch_qr_code(http_client).await?;
        let _ = qr_url_handler(qrcode_url).await?;
        let mut fetch_qr_code_base_url = WECHAT_BASE_URL.to_string();
        let qr_login_timeout = qr_login_timeout.unwrap_or(Duration::from_secs(120));
        let now = std::time::SystemTime::now();
        loop {
            if let Some(self_) = Self::from_account_state(&config).await? {
                return Ok(self_);
            }
            if now.elapsed()? > qr_login_timeout {
                return Err(anyhow!("QR_CODE_TIMEOUT"));
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
            match fetch_qr_code_status(http_client, &fetch_qr_code_base_url, &qrcode).await {
                Ok(QrCodeStatusResponse {
                    status,
                    redirect_host,
                    ilink_bot,
                    base_url,
                    ..
                }) => match status {
                    QrStatus::Wait | QrStatus::Scaned => {}
                    QrStatus::ScannedButRedirect => {
                        if let Some(host) = redirect_host.as_deref() {
                            fetch_qr_code_base_url = format!("https://{}", host);
                        }
                    }
                    QrStatus::Expired => {
                        return Err(anyhow!("QR_CODE_EXPIRED"));
                    }
                    QrStatus::Confirmed => {
                        info!("connected to wechat, account_id: {}", account_id);
                        let ILinkBot { bot_id, bot_token } =
                            ilink_bot.ok_or(anyhow!("ilink_bot not found"))?;
                        let account_state_path =
                            config.account_state_path().await?.join("config.json");
                        let self_ = Self {
                            account_id: account_id.clone(),
                            bot_id,
                            bot_token,
                            base_url: base_url.unwrap_or(WECHAT_BASE_URL.to_string()),
                        };
                        {
                            let json = serde_json::to_string_pretty(&self_)?;
                            tokio::fs::write(&account_state_path, &json).await?;
                        }
                        return Ok(self_);
                    }
                    QrStatus::Unknown => {}
                },
                Err(err) => {
                    warn!("{}", err);
                }
            }
        }
    }

    async fn from_account_state(config: &WechatConfig) -> crate::Result<Option<Self>> {
        let path = config.account_state_path().await?.join("config.json");
        if !path.exists() {
            return Ok(None);
        }
        let json = tokio::fs::read_to_string(path).await?;
        if json.is_empty() {
            return Ok(None);
        }
        let account = serde_json::from_str::<WechatAccount>(&json)?;
        Ok(Some(account))
    }
}
