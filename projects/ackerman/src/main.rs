use std::str::FromStr;

use futures_util::sink::SinkExt;
use reqwest::{
    Method, Url,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::handshake::client::Response;
use toml::Value;

use ackerman::{AckermanResult, GetChannelListResponse, GetGuildListResponse, QQBotWebsocket, SecretKey};

#[tokio::main]
async fn main() -> AckermanResult {
    let key = SecretKey::load("key.toml")?;
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
    let mut wss = QQBotWebsocket::link(&key).await.unwrap();
    // let a = wss.send(Message::from(""));
    println!("{:#?}", wss);
    Ok(())
}

