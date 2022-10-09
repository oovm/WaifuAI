use crate::{AckermanError, AckermanResult, SecretKey};
use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path, time::Duration};
use toml::from_str;

pub mod get_channel;
pub mod get_guild;
pub mod get_message;
pub mod secret_key;
