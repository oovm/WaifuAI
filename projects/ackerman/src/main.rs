use ackerman::qq_bots::{AckermanConfig, AckermanQQBot};
use qq_bot::{QQBotWebsocket, QQResult};

#[tokio::main]
async fn main() -> QQResult {
    let bot = AckermanQQBot::loading()?;
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
