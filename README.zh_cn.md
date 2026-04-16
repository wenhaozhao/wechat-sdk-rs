# wechat-sdk-rs

一个用于构建微信机器人流程的 Rust SDK，支持扫码登录、拉取消息和发送消息。

## 功能

- 扫码登录（`WechatClient::init`）
- 本地持久化账号与会话状态
- 拉取消息更新（`get_updates`）
- 发送文本/消息项（`send_message`）
- 输入中状态相关接口（`get_config`、`send_typing`、`send_typing_cannel`）

## 安装

```toml
[dependencies]
wechat-sdk-rs = "0.1.2"
tokio = { version = "1", features = ["full"] }
```

## 配置

`WechatConfig::from_env()` 会读取以下环境变量：

- `WECHAT_SDK_RS_ACCOUNT_ID`（必填）：本地状态目录的账号标识
- `WECHAT_SDK_RS_STATE_PATH`（可选）：状态根目录，默认 `~/.wechat`
- `WECHAT_SDK_RS_HTTP_TIMEOUT`（可选）：HTTP 超时（毫秒），默认 `2000`
- `WECHAT_SDK_RS_QR_LOGIN_TIMEOUT`（可选）：扫码登录超时（毫秒），默认 `120000`

状态文件会写入：

`{STATE_PATH}/{ACCOUNT_ID}/`

- `config.json`：bot 凭证和 base URL
- `session_context.json`：`context_token` 与更新游标

## 快速开始

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
            println!("打开链接并扫码登录: {}", url);
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
                    println!("收到: {}", text);
                    client
                        .send_message(user_id, format!("复读: {}", text), None)
                        .await?;
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
```

也可以直接运行示例：

```bash
WECHAT_SDK_RS_ACCOUNT_ID=demo cargo run --example hello
```

## API 概览

- 客户端生命周期
  - `WechatClient::new(config)`
  - `WechatClient::init(qr_url_handler)`
  - `WechatClient::account()`
- 消息
  - `get_updates() -> WechatMessages`
  - `send_message(to_user_id, msg_items, context_token)`
- 输入状态
  - `get_config(to_user_id, context_token) -> TypingTicket`
  - `send_typing(to_user_id, &TypingTicket)`
  - `send_typing_cannel(to_user_id, &TypingTicket)`

`send_message` 与 `get_config` 需要 `context_token`。  
传 `None` 时，SDK 会尝试从 `session_context.json` 读取（通常在接收过消息后可用）。

## 许可证

可任选其一：

- MIT（[LICENSE-MIT](./LICENSE-MIT)）
- Apache-2.0（[LICENSE-APACHE](./LICENSE-APACHE)）
