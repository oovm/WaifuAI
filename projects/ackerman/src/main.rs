use tokio::{
    join, select,
    time::{interval, Duration},
};
use tokio_tungstenite::tungstenite::{Error, Message};

use ackerman_qq::{
    restful::{GetChannelListResponse, GetGuildListResponse},
    AckermanResult, QQBotWebsocket, QQSecret,
};

#[tokio::main]
async fn main() -> AckermanResult {
    let key = QQSecret::load("key.toml")?;
    let bot =

    let mut wss = QQBotWebsocket::link(&key).await?;
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
