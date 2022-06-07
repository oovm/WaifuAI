use crate::AckermanResult;
use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path, time::Duration};
use toml::from_str;

mod get_channel;
mod get_guild;
mod get_message;
mod secret_key;
mod send_message;

pub use self::{get_channel::GetChannelListResponse, get_guild::GetGuildListResponse, secret_key::QQSecret};
