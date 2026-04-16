use crate::WECHAT_BASE_URL;
use derive_more::{Deref, Display, From, FromStr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

pub(super) async fn fetch_qr_code(http_client: &reqwest::Client) -> crate::Result<QrCodeResponse> {
    let url = format!("{}/ilink/bot/get_bot_qrcode", WECHAT_BASE_URL,);
    let resp = http_client
        .get(&url)
        .query(&[("bot_type", "3")])
        .send()
        .await?
        .json::<QrCodeResponse>()
        .await?;
    Ok(resp)
}

pub(super) async fn fetch_qr_code_status(
    http_client: &reqwest::Client,
    base_url: &str,
    qr_code: &QrCode,
) -> crate::Result<QrCodeStatusResponse> {
    let url = format!("{}/ilink/bot/get_qrcode_status", base_url,);
    let resp = http_client
        .get(&url)
        .query(&[("qrcode", qr_code.as_str())])
        .send()
        .await?
        .json()
        .await?;
    Ok(resp)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct QrCodeResponse {
    pub qrcode: QrCode,
    #[serde(rename = "qrcode_img_content")]
    pub qrcode_url: QrCodeUrl,
}

#[derive(Debug, Clone, From, FromStr, Deref, Serialize, Deserialize)]
pub(super) struct QrCode(String);

#[derive(Debug, Clone, From, FromStr, Deref, Serialize, Deserialize, Display)]
pub struct QrCodeUrl(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct QrCodeStatusResponse {
    pub status: QrStatus,
    #[serde(flatten)]
    pub ilink_bot: Option<ILinkBot>,
    #[serde(rename = "baseurl")]
    pub base_url: Option<String>,
    pub redirect_host: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ILinkBot {
    #[serde(rename = "ilink_bot_id")]
    pub bot_id: String,
    #[serde(rename = "bot_token")]
    pub bot_token: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, FromStr)]
#[display(rename_all = "snake_case")]
#[from_str(rename_all = "snake_case")]
pub(super) enum QrStatus {
    Wait,
    Scaned,
    ScannedButRedirect,
    Expired,
    Confirmed,
    #[default]
    Unknown,
}

impl<'de> Deserialize<'de> for QrStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let status = Option::<String>::deserialize(deserializer)
            .ok()
            .flatten()
            .and_then(|it| QrStatus::from_str(&it).ok())
            .unwrap_or_default();
        Ok(status)
    }
}

impl Serialize for QrStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
