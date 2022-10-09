use ackerman::{AckermanResult, GetChannelListResponse, GetGuildListResponse, SecretKey};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Error, Url,
};
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
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
    Ok(())
}
