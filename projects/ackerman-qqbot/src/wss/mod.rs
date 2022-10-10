use futures_util::SinkExt;
use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use reqwest::Method;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::{AckermanResult, QQBotSecret};

pub struct QQBotWebsocket {
    pub wss: WebSocketStream<MaybeTlsStream<TcpStream>>,
    key: QQBotSecret,
    connected: QQBotConnected,
}

#[derive(Deserialize, Debug)]
pub struct QQBotConnected {
    shards: u32,
    url: String,
    session_start_limit: SessionStartLimit,
}

#[derive(Deserialize, Debug)]
pub struct SessionStartLimit {
    max_concurrency: u32,
    remaining: u32,
    reset_after: u32,
    total: u32,
}

#[derive(Serialize, Debug)]
pub struct QQBotOperation {
    op: u32,
    d: QQBotOperationDispatch,
}

#[derive(Serialize, Debug)]
pub struct QQBotOperationDispatch {
    token: String,
    intents: u32,
    shard: Vec<u32>,
}

impl Debug for QQBotWebsocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QQBotWebsocket").field("config", self.wss.get_config()).field("connected", &self.connected).finish()
    }
}

impl QQBotWebsocket {
    pub async fn link(key: &QQBotSecret) -> AckermanResult<Self> {
        let url = Url::from_str("https://sandbox.api.sgroup.qq.com/gateway/bot")?;
        let value: QQBotConnected = key.as_request(Method::GET, url).send().await?.json().await?;
        let (wss, _) = connect_async(&value.url).await?;
        Ok(Self { wss, key: key.clone(), connected: value })
    }
    pub async fn identify(&mut self) -> AckermanResult<Self> {
        let op =
            QQBotOperation { op: 2, d: QQBotOperationDispatch { token: self.key.bot_token(), intents: 0, shard: vec![0] } };

        self.wss.send(Message::Text(op))
    }
}
