use ackerman::qq_bots::{AckermanConfig, AckermanQQBot};
use qq_bot::{QQBotWebsocket, QQResult};

#[tokio::main]
async fn main() -> QQResult {
    let key = AckermanConfig::load_toml("key.toml")?;
    let here = std::env::current_dir()?;
    let bot = AckermanQQBot::new(here, key)?;
    let mut wss = QQBotWebsocket::link(bot).await?;
    match wss.run().await {
        Ok(_) => {
            println!("已退出")
        }
        Err(e) => {
            println!("已退出: {:?}", e)
        }
    }
    Ok(())
}
