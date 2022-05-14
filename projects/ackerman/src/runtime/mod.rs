use std::fs::read_to_string;
use std::path::Path;
use serde::{Serialize, Deserialize};
use toml::from_str;
use crate::{AckermanError, AckermanResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretKey {
    bot_app_id: u64,
    bot_secret: String,
    bot_token: String,
}

impl SecretKey {
    pub fn load(path: impl AsRef<Path>) -> AckermanResult<Self> {
        Ok(from_str(&read_to_string(path)?)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMe {
    code: i32,
    message: String,

}