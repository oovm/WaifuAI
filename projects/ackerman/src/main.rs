use ackerman::{AckermanResult, GetGuildResponse, SecretKey};
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
    if key.has_channel_id() {
    }
    else {
        let out = GetGuildResponse::send(&key).await?;
        println!("{:#?}", out);
    }

    Ok(())
}
