use ackerman::{AckermanResult, GetChannelListResponse, GetGuildListResponse, GetMessageListResponse, SecretKey};
use futures_util::sink::SinkExt;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Error, Method, Url,
};
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use toml::Value;

#[tokio::main]
async fn main() -> AckermanResult {
    let key = SecretKey::load("projects/ackerman/key.toml").unwrap();
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
    let url = Url::from_str("https://sandbox.api.sgroup.qq.com/gateway/bot")?;
    let value: Value = key.as_request(Method::GET, url).send().await?.json().await?;
    println!("{:#?}", value);

    let (mut wss, response) = connect_async("wss://sandbox.api.sgroup.qq.com/websocket").await.unwrap();
    // let a = wss.send(Message::from(""));
    println!("{:#?}", response);
    println!("{:#?}", wss);

    Ok(())
}
