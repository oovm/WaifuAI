use ackerman::{AckermanResult, GetGuildResponse, SecretKey};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Error, Url,
};
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
use toml::Value;

fn root_url() -> Url {
    Url::from_str("https://sandbox.api.sgroup.qq.com/").unwrap()
}

#[tokio::main]
async fn main() -> AckermanResult {
    let key = SecretKey::load("projects/ackerman/key.toml").unwrap();
    // GET /guilds/{guild_id}/channels

    let request_url = format!("https://sandbox.api.sgroup.qq.com/guilds/{guild_id}/channels", guild_id = key.guild_id);
    println!("{}", request_url);
    let response = Client::default()
        .get(request_url)
        .header(AUTHORIZATION, format!("Bot {}.{}", key.bot_app_id, key.bot_secret))
        .send()
        .await?;

    let out: GetGuildResponse = response.json().await?;

    println!("{:#?}", out);
    Ok(())
}
