use futures_util::sink::SinkExt;
use tokio::time::{interval, Duration};

use ackerman_qq::{
    restful::{GetChannelListResponse, GetGuildListResponse},
    AckermanResult, QQBotSecret, QQBotWebsocket,
};

#[tokio::main]
async fn main() -> AckermanResult {
    let key = QQBotSecret::load("key.toml")?;
    if key.guild_id() == 0 {
        let out = GetGuildListResponse::send(&key).await?;
        println!("可行的频道有:");
        for item in out.items {
            println!("{}: {}", item.name, item.id)
        }
        return Ok(());
    }
    if key.channel_id() == 0 {
        let out = GetChannelListResponse::send(&key).await?;
        println!("可行的子频道有: {:#?}", out);
        for item in out.items {
            println!("{}: {}", item.name, item.id)
        }
        return Ok(());
    }
    let mut wss = QQBotWebsocket::link(&key).await?;
    let mut heartbeat = interval(Duration::from_secs_f32(30.0));

    heartbeat.tick().await;
    loop {
        tokio::select! {
            Some(event) = wss.next() => {
                wss.dispatch(event).await?;
            }
            _ = heartbeat.tick() => {
                 wss.send_heartbeat().await?;
            },
        }
    }
}
