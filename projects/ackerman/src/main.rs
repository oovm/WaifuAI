use std::str::FromStr;

use futures_util::sink::SinkExt;
use reqwest::{Method, Url};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::handshake::client::Response, MaybeTlsStream, WebSocketStream};
use toml::Value;

use ackerman::{AckermanResult, GetChannelListResponse, GetGuildListResponse, QQBotSecret, QQBotWebsocket};

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
    // let out = GetMessageListResponse::send(&key).await?;
    // println!("可行的子频道有: {:#?}", out);
    let mut wss = QQBotWebsocket::link(&key).await?;
    let out = wss.identify().await?;

    Ok(())
}
