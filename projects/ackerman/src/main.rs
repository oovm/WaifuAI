use ackerman::qq_bots::AckermanQQBot;
use tokio::{
    select,
    time::{interval, Duration},
};

use qq_bot::{AckermanResult, QQBotWebsocket, QQSecret};

#[tokio::main]
async fn main() -> AckermanResult {
    let key = QQSecret::load_toml("key.toml")?;
    let here = std::env::current_dir()?;
    let bot = AckermanQQBot { secret: key, here };
    bot.ensure_path()?;

    let mut wss = QQBotWebsocket::link(bot).await?;
    let mut heartbeat = interval(Duration::from_secs_f32(30.0));
    wss.send_identify().await?;
    loop {
        select! {
            listen = wss.next() => {
                match listen {
                    Some(event) =>{
                        wss.dispatch(event).await?;
                    }
                    None => {
                        break
                    }
                }
            },
            _ = heartbeat.tick() => {
                 if wss.closed {
                    break
                 }
                 else {
                    wss.send_heartbeat().await?;
                }
            },
        }
    }
    Ok(())
}
