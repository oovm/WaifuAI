use chrono::Utc;
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
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::{AckermanResult, QQBotSecret};

pub struct QQBotWebsocket {
    pub wss: WebSocketStream<MaybeTlsStream<TcpStream>>,
    key: QQBotSecret,
    connected: QQBotConnected,
    pub closed: bool,
    pub heartbeat_interval: u32,
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
    d: QQBotOperationUnion,
}

impl QQBotOperation {
    pub fn dispatched(self) -> QQBotOperationDispatch {
        match self.d {
            QQBotOperationUnion::Dispatch(d) => d,
            QQBotOperationUnion::Boolean(_) => Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum QQBotOperationUnion {
    Dispatch(QQBotOperationDispatch),
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QQBotOperationDispatch {
    #[serde(default)]
    token: String,
    #[serde(default)]
    intents: u32,
    #[serde(default)]
    shard: Vec<u32>,
    #[serde(default)]
    heartbeat_interval: u32,
}
impl Default for QQBotOperationUnion {
    fn default() -> Self {
        Self::Dispatch(Default::default())
    }
}

impl Default for QQBotOperationDispatch {
    fn default() -> Self {
        Self { token: "".to_string(), intents: 0, shard: vec![], heartbeat_interval: 40000 }
    }
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
        Ok(Self { wss, key: key.clone(), connected: value, closed: false, heartbeat_interval: 40000 })
    }
    pub async fn next_event(&mut self) -> AckermanResult {
        let op: QQBotOperation = match self.wss.next().await {
            Some(s) => {
                let ss = s?;
                match ss {
                    Message::Text(s) => from_str(&s)?,
                    Message::Close(_) => {
                        self.closed = true;
                        println!("链接已关闭");
                        return Ok(());
                    }
                    _ => unreachable!("{:#?}", ss),
                }
            }
            None => return Ok(()),
        };
        println!("[{}] 协议 {}", Utc::now(), op.op);
        match op.op {
            9 => {
                println!("    重连参数有误");
            }
            10 => {
                self.heartbeat_interval = op.dispatched().heartbeat_interval;
                println!("    重设心跳间隔为 {}", self.heartbeat_interval);
            }
            _ => {
                println!("{:#?}", op);
            }
        }

        Ok(())
    }
    pub async fn send_identify(&mut self) -> AckermanResult<()> {
        let op = QQBotOperation {
            //
            op: 2,
            d: QQBotOperationUnion::Dispatch(QQBotOperationDispatch {
                token: self.key.bot_token(),
                intents: 0,
                shard: vec![0],
                ..Default::default()
            }),
        };
        self.wss.send(Message::Text(to_string(&op)?)).await?;
        Ok(())
    }
}
