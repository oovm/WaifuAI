use ackerman::qq_bots::AckermanQQBot;
use qq_bot::{QQBotWebsocket, QQResult, QQSecret};

#[tokio::main]
async fn main() -> QQResult {
    let key = QQSecret::load_toml("key.toml")?;
    let here = std::env::current_dir()?;
    let bot = AckermanQQBot::new(here, key)?;
    let mut wss = QQBotWebsocket::link(bot).await?;
    wss.run().await
}
