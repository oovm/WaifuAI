use std::{
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
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::{AckermanResult, QQBotSecret};

pub use self::{heartbeat_event::HeartbeatEvent, message_event::MessageEvent, ready_event::LoginEvent};

mod heartbeat_event;
mod message_event;
mod ready_event;

pub struct QQBotWebsocket {
    pub wss: WebSocketStream<MaybeTlsStream<TcpStream>>,
    key: QQBotSecret,
    connected: QQBotConnected,
    pub closed: bool,
    pub heartbeat_interval: u64,
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
    d: EventDispatcher,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventDispatcher {
    Message(MessageEvent),
    Dispatch(QQBotOperationDispatch),
    HeartbeatEvent(HeartbeatEvent),
    LoginReadyEvent(LoginEvent),
    MaybeFail(bool),
    Integer(u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QQBotOperationDispatch {
    token: String,
    intents: u32,
    shard: Vec<u32>,
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::MaybeFail(false)
    }
}

impl Default for QQBotOperationDispatch {
    fn default() -> Self {
        Self { token: "".to_string(), intents: 0, shard: vec![] }
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
    pub async fn next(&mut self) -> Option<Result<Message, Error>> {
        self.wss.next().await
    }
    pub async fn dispatch(&mut self, event: Result<Message, Error>) -> AckermanResult {
        let received: QQBotOperation = match event? {
            Message::Text(s) => match from_str(&s) {
                Ok(o) => o,
                Err(e) => {
                    let json: Value = from_str(&s)?;
                    print!("未知错误 {:#?}", e);
                    panic!("{:#?}", json);
                }
            },
            Message::Close(_) => {
                self.closed = true;
                println!("链接已关闭");
                return Ok(());
            }
            _ => unreachable!(),
        };
        let mut bot = MyBot {};
        let reply = match received.op {
            0 => match received.d {
                EventDispatcher::Dispatch(v) => {
                    println!("    鉴权成功, 登陆为 {:?}", v);
                    None
                }
                EventDispatcher::Message(msg) => bot.on_message(msg),
                EventDispatcher::LoginReadyEvent(msg) => bot.on_login(msg),
                _ => unreachable!(),
            },
            9 => {
                println!("[{}] 协议 {}", Utc::now().format("%F %H:%M:%S"), received.op);
                println!("    鉴权参数有误");
                None
            }
            10 => {
                println!("[{}] 协议 {}", Utc::now().format("%F %H:%M:%S"), received.op);
                self.heartbeat_id = received.s;
                match received.d {
                    EventDispatcher::HeartbeatEvent(time) => {
                        self.heartbeat_interval = time.heartbeat_interval;
                    }
                    _ => unreachable!(),
                }

                println!("    重设心跳间隔为 {}", self.heartbeat_interval);
                None
            }
            // 接收到心跳包, 无参数
            11 => None,
            _ => {
                println!("[{}] 协议 {}", Utc::now().format("%F %H:%M:%S"), received.op);
                println!("未知协议 {:#?}", received);
                None
            }
        };
        match reply {
            Some(s) => self.send(&s).await?,
            None => {}
        }
        Ok(())
    }
    pub async fn send(&mut self, operator: &QQBotOperation) -> AckermanResult<()> {
        self.wss.send(Message::Text(to_string(&operator)?)).await?;
        Ok(())
    }
    pub async fn send_heartbeat(&mut self) -> AckermanResult<()> {
        println!("[{}] 协议 1", Utc::now().format("%F %H:%M:%S"));
        let protocol = QQBotOperation {
            op: 1,
            s: 0,
            t: "".to_string(),
            d: EventDispatcher::Integer(self.heartbeat_id),
            id: "".to_string(),
        };
        self.send(&protocol).await?;
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
            d: EventDispatcher::Dispatch(QQBotOperationDispatch {
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

pub struct MyBot {}

#[async_trait]
#[allow(unused_variables)]
pub trait QQBotProtocol {
    async fn on_login(&mut self, event: LoginEvent) -> AckermanResult {
        println!("    登录成功, 登陆为 {:?}", event.user.username);
        Ok(())
    }
    async fn on_message(&mut self, event: MessageEvent) -> AckermanResult {
        println!("    收到消息, 发送者为 {:?}", event.author.username);
        Ok(())
    }
}

#[async_trait]
impl QQBotProtocol for MyBot {
    async fn on_message(&mut self, event: MessageEvent) -> AckermanResult {
        // event.content
        println!("收到消息 {:#?}", event);
        Ok(())
    }
}
