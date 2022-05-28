use std::{
    borrow::Borrow,
    fmt::{Debug, Formatter},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use futures_util::{SinkExt, StreamExt};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::{AckermanError, AckermanResult, QQBotSecret};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct QQBotOperation {
    op: u32,
    d: QQBotOperationDispatch,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct QQBotOperationDispatch {
    #[serde(default)]
    token: String,
    #[serde(default)]
    intents: u32,
    #[serde(default)]
    shard: Vec<u32>,
    #[serde(default)]
    heartbeat_interval: i64,
}

impl Debug for QQBotWebsocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tcp_stream = match self.wss.get_ref() {
            MaybeTlsStream::Plain(s) => s.peer_addr().unwrap(),
            MaybeTlsStream::NativeTls(t) => t.get_ref().get_ref().get_ref().peer_addr().unwrap(),
            _ => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        };
        f.debug_struct("QQBotWebsocket")
            .field("config", self.wss.get_config())
            .field("socket", &tcp_stream)
            .field("connected", &self.connected)
            .finish()
    }
}

impl QQBotWebsocket {
    pub async fn link(key: &QQBotSecret) -> AckermanResult<Self> {
        let url = Url::from_str("https://sandbox.api.sgroup.qq.com/gateway/bot")?;
        let value: QQBotConnected = key.as_request(Method::GET, url).send().await?.json().await?;
        let (wss, _) = connect_async(&value.url).await?;
        Ok(Self { wss, key: key.clone(), connected: value })
    }
    pub async fn send_hello(&mut self) -> AckermanResult<u64> {
        let op = QQBotOperation {
            //
            op: 1,
            d: QQBotOperationDispatch {
                token: self.key.bot_token(),
                // https://bot.q.qq.com/wiki/develop/api/gateway/intents.html
                intents: 1 << 9 + 1 << 26,
                shard: vec![0, 4],
                ..Default::default()
            },
        };
        self.wss.send(Message::Text(to_string(&op)?)).await?;
    }
    pub async fn next_event(&mut self) -> AckermanResult {
        let op: QQBotOperation = match self.wss.next().await {
            Some(s) => match s? {
                Message::Text(s) => from_str(&s)?,
                _ => return Err(AckermanError::UnknownError),
            },
            None => return Ok(()),
        };

        let op: QQBotOperation = match self.wss.next().await {
            Some(Ok(Message::Text(s))) => from_str(&s)?,
            _ => return Err(AckermanError::UnknownError),
        };
        println!("{:#?}", op);
        Ok(())
    }

    pub async fn send_identify(&mut self) -> AckermanResult<u64> {
        let op = QQBotOperation {
            //
            op: 2,
            d: QQBotOperationDispatch { token: self.key.bot_token(), intents: 0, shard: vec![0], ..Default::default() },
        };
        self.wss.send(Message::Text(to_string(&op)?)).await?;
        #[derive(Deserialize)]
        struct HeartbeatResponse {
            pub op: i64,
            pub d: HeartbeatInterval,
        }
        #[derive(Deserialize)]
        struct HeartbeatInterval {
            pub heartbeat_interval: i64,
        }
        match self.wss.next().await {
            Some(Ok(Message::Text(s))) => {
                let json: serde_json::Value = from_str(&s)?;
                println!("{:#?}", json);
                todo!()
                // Ok(json.d.heartbeat_interval as u64)
            }
            _ => Err(AckermanError::UnknownError),
        }
    }
}
