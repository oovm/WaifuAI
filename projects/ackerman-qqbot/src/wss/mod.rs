use std::{
    borrow::Borrow,
    fmt::{Debug, Formatter},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::{restful::MessageItem, AckermanResult, QQBotSecret};

pub struct QQBotWebsocket {
    pub wss: WebSocketStream<MaybeTlsStream<TcpStream>>,
    key: QQBotSecret,
    connected: QQBotConnected,
    pub closed: bool,
    pub heartbeat_interval: u32,
    pub heartbeat_id: u32,
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
    #[serde(default)]
    d: QQBotOperationUnion,
    #[serde(default)]
    s: u32,
    #[serde(default)]
    t: String,
    #[serde(default)]
    id: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub bot: bool,
}

impl QQBotOperation {
    pub fn dispatched(self) -> QQBotOperationDispatch {
        match self.d {
            QQBotOperationUnion::Dispatch(d) => d,
            QQBotOperationUnion::Boolean(_) => Default::default(),
            QQBotOperationUnion::Integer(_) => Default::default(),
            QQBotOperationUnion::Message(_) => Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum QQBotOperationUnion {
    Message(MessageItem),
    Dispatch(QQBotOperationDispatch),
    Boolean(bool),
    Integer(u32),
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
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub user: User,
}

impl Default for QQBotOperationUnion {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

impl Default for QQBotOperationDispatch {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            intents: 0,
            shard: vec![],
            heartbeat_interval: 40000,
            version: 0,
            session_id: "".to_string(),
            user: Default::default(),
        }
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
        Ok(Self { wss, key: key.clone(), connected: value, closed: false, heartbeat_interval: 40000, heartbeat_id: 0 })
    }
    pub async fn next_event(&mut self) -> AckermanResult {
        let received: QQBotOperation = match self.wss.next().await {
            Some(s) => {
                let ss = s?;
                match ss {
                    Message::Text(s) => {
                        let debug: Value = from_str(&s)?;
                        println!("{:#?}", debug);
                        from_str(&s)?
                    }
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
        println!("[{}] 协议 {}", Utc::now().format("%F %H:%M:%S"), received.op);
        match received.op {
            0 => match received.d {
                QQBotOperationUnion::Dispatch(v) => {
                    println!("    鉴权成功, 登陆为 {:?}", v.user.username);
                }
                QQBotOperationUnion::Boolean(v) => {
                    unreachable!("{:?}", v)
                }
                QQBotOperationUnion::Integer(v) => {
                    unreachable!("{:?}", v)
                }
                QQBotOperationUnion::Message(msg) => {
                    println!("收到消息 {:#?}", msg);
                }
            },
            9 => {
                println!("    鉴权参数有误");
            }
            10 => {
                self.heartbeat_id = received.s;
                self.heartbeat_interval = received.dispatched().heartbeat_interval;
                println!("    重设心跳间隔为 {}", self.heartbeat_interval);
            }
            // 接收到心跳包
            11 => {
                self.heartbeat_id = received.s;
                println!("    接受心跳包 {}", self.heartbeat_id);
            }
            _ => {
                println!("未知协议 {:#?}", received);
            }
        }
        Ok(())
    }
    pub async fn send_heartbeat(&mut self) -> AckermanResult<()> {
        println!("[{}] 协议 1", Utc::now().format("%F %H:%M:%S"));
        let protocol = QQBotOperation {
            op: 1,
            s: 0,
            t: "".to_string(),
            d: QQBotOperationUnion::Integer(self.heartbeat_id),
            id: "".to_string(),
        };
        self.wss.send(Message::Text(to_string(&protocol)?)).await?;
        println!("    发送心跳包 {}", self.heartbeat_id);
        Ok(())
    }
    pub async fn send_identify(&mut self) -> AckermanResult<()> {
        println!("[{}] 协议 2", Utc::now().format("%F %H:%M:%S"));
        let intents = 1 << 9 | 1 << 10 | 1 << 26 | 1 << 30;
        let protocol = QQBotOperation {
            op: 2,
            s: 0,
            t: "".to_string(),
            d: QQBotOperationUnion::Dispatch(QQBotOperationDispatch {
                token: self.key.bot_token(),
                intents,
                shard: vec![0, 1],
                ..Default::default()
            }),
            id: "".to_string(),
        };
        self.wss.send(Message::Text(to_string(&protocol)?)).await?;
        println!("    首次连接鉴权");
        println!("    监听掩码 {:0X}", intents);
        Ok(())
    }
}
