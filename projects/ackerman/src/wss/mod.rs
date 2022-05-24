use std::str::FromStr;
use reqwest::Method;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use toml::Value;
use url::Url;
use crate::{AckermanResult, SecretKey};

#[derive(Debug)]
pub struct QQBotWebsocket {
    wss: WebSocketStream<MaybeTlsStream<TcpStream>>,
    info: QQBotConnected
}

impl QQBotWebsocket {
    pub async fn link(key: &SecretKey) -> AckermanResult<Self> {
        let url = Url::from_str("https://sandbox.api.sgroup.qq.com/gateway/bot")?;
        let value: Value = key.as_request(Method::GET, url).send().await?.json().await?;
        println!("{:#?}", value);

        let (wss, response) = connect_async("wss://sandbox.api.sgroup.qq.com/websocket").await?;
        Ok(Self {
            wss,
            info: QQBotConnected { shards: 0, url: "".to_string() }
        })
    }
}

pub struct QQBotConnected {
    shards: u32,
    url: String,
}