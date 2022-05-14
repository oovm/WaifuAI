use std::path::PathBuf;
use std::str::FromStr;
use serde::Deserialize;
use reqwest::Error;
use toml::Value;
use ackerman::SecretKey;




#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = SecretKey::load("projects/ackerman/key.toml").unwrap();
    let request_url = format!("https://api.sgroup.qq.com/users/@me");
    let response = reqwest::get(&request_url).await?;

    let out: Value = response.json().await?;

    println!("{:#?}", out);
    Ok(())
}