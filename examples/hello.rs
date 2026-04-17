use std::ops::Deref;
use std::time::Duration;

use wechat_sdk::client::message::{MessageItem, MessageItemValue, TextItem, WechatMessage};
use wechat_sdk::client::{WechatClient, WechatConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let config = WechatConfig::from_env()?;
    let client = WechatClient::new(config)
        .await?
        .init(async |url| {
            println!("open url {} and scan qr-code for login", url);
            Ok(())
        })
        .await?;
    loop {
        let messages = client.get_updates().await?;
        if !messages.is_empty() {
            for WechatMessage {
                items,
                ..
            } in messages.deref()
            {
                for MessageItem { value, .. } in items.deref() {
                    if let MessageItemValue::Text {
                        text_item: TextItem { text },
                    } = value
                    {
                        println!("recv: {}", text);
                        let _ = client.send_message(format!("I repeat: {}", text)).await?;
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
