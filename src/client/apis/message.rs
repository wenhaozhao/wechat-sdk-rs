use crate::account::WechatUserId;
use crate::client::WechatConfig;
use derive_more::{Deref, DerefMut, Display, From, FromStr};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingTicket(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatMessage {
    #[serde(rename = "client_id")]
    pub message_id: MessageId,
    pub context_token: ContextToken,
    pub from_user_id: FromUserId,
    pub to_user_id: ToUserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    wechat_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<usize>,
    pub message_type: MessageType,
    pub message_state: i64,
    #[serde(rename = "item_list")]
    pub items: MessageItems,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time_ms: Option<i64>,
}

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
    Display,
)]
pub struct MessageId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextToken(String);

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Deref, Default)]
pub struct FromUserId(WechatUserId);

impl<U> From<U> for FromUserId
where
    U: Into<WechatUserId>,
{
    fn from(value: U) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Deref, Default)]
pub struct ToUserId(WechatUserId);

impl<U> From<U> for ToUserId
where
    U: Into<WechatUserId>,
{
    fn from(value: U) -> Self {
        Self(value.into())
    }
}

impl From<&ToUserId> for FromUserId {
    fn from(value: &ToUserId) -> Self {
        Self(value.deref().clone())
    }
}

impl From<&FromUserId> for ToUserId {
    fn from(value: &FromUserId) -> Self {
        Self(value.deref().clone())
    }
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    User = 1,
    Bot = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Deref, DerefMut)]
pub struct MessageItems(pub Vec<MessageItem>);

impl<Item> From<Item> for MessageItems
where
    Item: Into<MessageItem>,
{
    fn from(value: Item) -> Self {
        Self(vec![value.into()])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageItem {
    #[serde(rename = "type")]
    value_type: MessageItemValueType,
    #[serde(flatten)]
    pub value: MessageItemValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_msg: Option<RefMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_ms: Option<i64>,
}

impl<V> From<V> for MessageItem
where
    V: Into<MessageItemValue>,
{
    fn from(value: V) -> Self {
        let value = value.into();
        Self {
            value_type: (&value).into(),
            value,
            ref_msg: None,
            is_completed: None,
            create_time_ms: None,
            update_time_ms: None,
        }
    }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum MessageItemValueType {
    Text = 1,
    Image = 2,
    Voice = 3,
    File = 4,
    Video = 5,
    Unsupported = 8,
}
#[derive(Debug, Clone, Serialize, Deserialize, From)]
#[serde(untagged)]
pub enum MessageItemValue {
    //#[serde(rename = "1")]
    Text { text_item: TextItem },
    //#[serde(rename = "2")]
    Image { image_item: ImageItem },
    //#[serde(rename = "3")]
    Voice { voice_item: VoiceItem },
    //#[serde(rename = "4")]
    File { file_item: FileItem },
    //#[serde(rename = "5")]
    Video { video_item: VideoItem },
    Unsupported { unsupported_item: UnsupportedItem },
}

impl From<&MessageItemValue> for MessageItemValueType {
    fn from(value: &MessageItemValue) -> Self {
        match value {
            MessageItemValue::Text { .. } => Self::Text,
            MessageItemValue::Image { .. } => Self::Image,
            MessageItemValue::Voice { .. } => Self::Video,
            MessageItemValue::File { .. } => Self::File,
            MessageItemValue::Video { .. } => Self::Video,
            MessageItemValue::Unsupported { .. } => Self::Unsupported,
        }
    }
}

impl<S> From<S> for MessageItemValue
where
    S: Display,
{
    fn from(value: S) -> Self {
        MessageItemValue::Text {
            text_item: TextItem {
                text: value.to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextItem {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageItem {
    pub hd_size: usize,
    pub mid_size: usize,
    pub thumb_height: u32,
    pub thumb_width: u32,
    pub thumb_size: usize,
    #[serde(rename = "aeskey")]
    pub aes_key: String,
    pub media: CdnMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceItem {
    pub bits_per_sample: u8,
    pub encode_type: u16,
    pub playtime: u64,
    pub sample_rate: u32,
    pub media: CdnMedia,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub file_name: String,
    pub len: usize,
    pub md5: String,
    pub media: CdnMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoItem {
    pub play_length: u64,
    pub thumb_height: u32,
    pub thumb_width: u32,
    pub video_size: usize,
    pub video_md5: String,
    pub media: CdnMedia,
    pub thumb_media: CdnMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsupportedItem {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefMessage {
    pub message_item: Box<MessageItem>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnMedia {
    pub aes_key: String,
    pub encrypt_query_param: String,
    pub full_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub(super) struct SessionContext {
    pub context_token: Option<ContextToken>,
    pub get_updates_buf: Option<String>,
}

impl SessionContext {
    async fn path(config: &WechatConfig) -> crate::Result<PathBuf> {
        let session_ctx_path = config
            .account_state_path()
            .await?
            .join("session_context.json");
        Ok(session_ctx_path)
    }
    pub(super) async fn load(config: &WechatConfig) -> crate::Result<Self> {
        let path = Self::path(config).await?;
        let session_ctx = if path.exists() {
            tokio::fs::read_to_string(&path)
                .await
                .ok()
                .and_then(|json| serde_json::from_str::<SessionContext>(&json).ok())
                .unwrap_or_default()
        } else {
            Default::default()
        };
        Ok(session_ctx)
    }

    pub(super) async fn flush(self, config: &WechatConfig) -> crate::Result<Self> {
        let path = Self::path(config).await?;
        let json = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(path, json).await?;
        Ok(self)
    }
}
