use std::{
    collections::BTreeMap,
    fs,
    fs::read_to_string,
    io::Write,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::http::Method;

use qq_bot::{restful::SendMessageRequest, wss::MessageEvent, QQBotProtocol, QQResult, QQSecret, RequestBuilder, Url};

pub use self::image_request::NovelAIRequest;

mod image_request;
