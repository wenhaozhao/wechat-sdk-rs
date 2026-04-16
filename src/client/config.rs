use crate::account::WechatAccountId;
use crate::client::WechatConfig;
use anyhow::anyhow;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

impl WechatConfig {
    pub async fn account_state_path(&self) -> crate::Result<PathBuf> {
        let path = self.state_path.join(self.account_id.deref());
        if path.exists() {
            if path.is_dir() {
                Ok(path)
            } else {
                Err(anyhow!("path: {} is not dir", path.display()))
            }
        } else {
            tokio::fs::create_dir_all(&path).await?;
            Ok(path)
        }
    }
}
impl WechatConfig {
    pub fn from_env() -> crate::Result<Self> {
        Ok(Self {
            state_path: state_path_from_env()?,
            account_id: account_id_from_env()?,
            http_timeout: http_timeout_from_env().ok(),
            qr_login_timeout: qr_login_timeout_from_env().ok(),
            http_api_get_updates_timeout: None,
        })
    }
}

fn state_path_from_env() -> crate::Result<PathBuf> {
    const ENV_NAME: &str = "WECHAT_SDK_RS_STATE_PATH";
    if let Some(path) = std::env::var(ENV_NAME)
        .ok()
        .and_then(|it| PathBuf::from_str(&it).ok())
    {
        Ok(path)
    } else {
        let base_path = dirs::home_dir()
            .or_else(|| dirs::template_dir())
            .ok_or(anyhow!("cannot create wechat state path"))?;
        let dst = base_path.join(".wechat");
        Ok(dst)
    }
}

fn account_id_from_env() -> crate::Result<WechatAccountId> {
    const ENV_NAME: &str = "WECHAT_SDK_RS_ACCOUNT_ID";
    let val = std::env::var(ENV_NAME).map_err(|_| anyhow!("env `{ENV_NAME}` not found"))?;
    Ok(val.into())
}

fn http_timeout_from_env() -> crate::Result<Duration> {
    const ENV_NAME: &str = "WECHAT_SDK_RS_HTTP_TIMEOUT";
    Ok(std::env::var(ENV_NAME)
        .ok()
        .and_then(|it| it.parse().ok())
        .map(|val| Duration::from_millis(val))
        .unwrap_or(Duration::from_millis(2000)))
}

fn qr_login_timeout_from_env() -> crate::Result<Duration> {
    const ENV_NAME: &str = "WECHAT_SDK_RS_QR_LOGIN_TIMEOUT";
    Ok(std::env::var(ENV_NAME)
        .ok()
        .and_then(|it| it.parse().ok())
        .map(|val| Duration::from_millis(val))
        .unwrap_or(Duration::from_secs(120)))
}

#[cfg(test)]
mod tests {
    use crate::client::WechatConfig;

    #[test]
    fn test_from_env() {
        let config = WechatConfig::from_env().unwrap();
        let json = serde_json::to_string_pretty(&config).unwrap();
        println!("{}", json);
    }
}
