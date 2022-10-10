use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use reqwest::Method;
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::{AckermanResult, SecretKey};

pub struct QQBotWebsocket {
    wss: WebSocketStream<MaybeTlsStream<TcpStream>>,
    connected: QQBotConnected,
}

impl Debug for QQBotWebsocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QQBotWebsocket").field("config", self.wss.get_config()).field("connected", &self.connected).finish()
    }
}

impl QQBotWebsocket {
    pub async fn link(key: &SecretKey) -> AckermanResult<Self> {
        let url = Url::from_str("https://sandbox.api.sgroup.qq.com/gateway/bot")?;
        let value: QQBotConnected = key.as_request(Method::GET, url).send().await?.json().await?;
        println!("{:#?}", value);

        let (wss, _) = connect_async(&value.url).await?;
        Ok(Self { wss, connected: value })
    }
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
