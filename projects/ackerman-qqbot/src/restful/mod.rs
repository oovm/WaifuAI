use std::str::FromStr;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{QQBotProtocol, QQResult, QQSecret};

pub use self::{get_channel::GetChannelListResponse, get_guild::GetGuildListResponse, send_message::SendMessageRequest};

mod get_channel;
mod get_guild;
mod get_message;
mod send_message;
