# wechat-sdk-rs

Rust SDK for building WeChat bot workflows with QR-code login and simple message APIs.

## Features

- QR-code login (`WechatClient::init`)
- Local account/session persistence
- Pull updates (`get_updates`)
- Send text/message items (`send_message`)
- Typing status APIs (`get_config`, `send_typing`, `send_typing_cannel`)

## Installation

```toml
[dependencies]
wechat-sdk-rs = "0.1.2"
tokio = { version = "1", features = ["full"] }
```

## Configuration

`WechatConfig::from_env()` reads:

- `WECHAT_SDK_RS_ACCOUNT_ID` (required): account namespace for local state
- `WECHAT_SDK_RS_STATE_PATH` (optional): state root path, default `~/.wechat`
- `WECHAT_SDK_RS_HTTP_TIMEOUT` (optional): HTTP timeout in milliseconds, default `2000`
- `WECHAT_SDK_RS_QR_LOGIN_TIMEOUT` (optional): QR login timeout in milliseconds, default `120000`

State files are written under:

`{STATE_PATH}/{ACCOUNT_ID}/`

- `config.json`: bot credentials and base URL
- `session_context.json`: `context_token` and update cursor

## Quick Start

```rust
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
            println!("Open this URL and scan QR code: {}", url);
            Ok(())
        })
        .await?;

    loop {
        let messages = client.get_updates().await?;
        for WechatMessage {
            from_user_id: user_id,
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
                    client
                        .send_message(user_id, format!("I repeat: {}", text), None)
                        .await?;
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
```

You can also run the bundled example:

```bash
WECHAT_SDK_RS_ACCOUNT_ID=demo cargo run --example hello
```

## Public API Overview

- Client lifecycle
  - `WechatClient::new(config)`
  - `WechatClient::init(qr_url_handler)`
  - `WechatClient::account()`
- Messaging
  - `get_updates() -> WechatMessages`
  - `send_message(to_user_id, msg_items, context_token)`
- Typing
  - `get_config(to_user_id, context_token) -> TypingTicket`
  - `send_typing(to_user_id, &TypingTicket)`
  - `send_typing_cannel(to_user_id, &TypingTicket)`

`send_message` and `get_config` require a `context_token`.  
If you pass `None`, SDK tries to load it from `session_context.json` (usually set after receiving messages).

## License

Licensed under either:

- MIT ([LICENSE-MIT](./LICENSE-MIT))
- Apache-2.0 ([LICENSE-APACHE](./LICENSE-APACHE))
