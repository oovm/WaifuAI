use crate::QQBotProtocol;
use async_trait::async_trait;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
use std::{
    fmt::{Debug, Formatter},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::{AckermanResult, QQSecret};

pub use self::{
    emoji_event::EmojiEvent,
    heartbeat_event::HeartbeatEvent,
    message_event::{MessageAttachment, MessageEvent},
    ready_event::LoginEvent,
};

mod emoji_event;
mod heartbeat_event;
mod message_event;
mod ready_event;

pub struct QQBotWebsocket<T>
where
    T: QQBotProtocol,
{
    bot: T,
    wss: WebSocketStream<MaybeTlsStream<TcpStream>>,
    connected: QQBotConnected,
    heartbeat_id: u32,
    pub closed: bool,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventDispatcher {
    Message(MessageEvent),
    Dispatch(QQBotOperationDispatch),
    HeartbeatEvent(HeartbeatEvent),
    LoginReadyEvent(LoginEvent),
    EmojiEvent(EmojiEvent),
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

impl<T> Debug for QQBotWebsocket<T>
where
    T: QQBotProtocol,
{
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

impl<T> QQBotWebsocket<T>
where
    T: QQBotProtocol,
{
    pub async fn link(bot: T) -> AckermanResult<Self> {
        let url = Url::from_str("https://sandbox.api.sgroup.qq.com/gateway/bot")?;
        let request = bot.build_request(Method::GET, url);
        let connected: QQBotConnected = request.send().await?.json().await?;
        let (wss, _) = connect_async(&connected.url).await?;
        Ok(Self { wss, bot, connected, heartbeat_id: 0, closed: false })
    }
    pub async fn relink(&mut self) -> AckermanResult {
        let url = Url::from_str("https://sandbox.api.sgroup.qq.com/gateway/bot")?;
        let request = self.bot.build_request(Method::GET, url);
        let connected: QQBotConnected = request.send().await?.json().await?;
        let (wss, _) = connect_async(&connected.url).await?;
        self.wss = wss;
        self.closed = false;
        Ok(())
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
        match received.op {
            0 => match received.d {
                EventDispatcher::Dispatch(v) => {
                    println!("    鉴权成功, 登陆为 {:?}", v);
                }
                EventDispatcher::Message(msg) => self.bot.on_message(msg).await?,
                EventDispatcher::LoginReadyEvent(msg) => self.bot.on_login_success(msg).await?,
                EventDispatcher::EmojiEvent(msg) => self.bot.on_emoji(msg).await?,
                _ => unreachable!(),
            },
            // 要求重新链接
            7 => self.relink().await?,
            9 => self.bot.on_login_failure().await?,
            10 => match received.d {
                EventDispatcher::HeartbeatEvent(time) => {
                    self.heartbeat_id = received.s;
                    self.bot.on_connected(time).await?;
                }
                _ => unreachable!(),
            },
            // 接收到心跳包, 无参数
            11 => {}
            _ => {
                println!("[{}] 协议 {}", Utc::now().format("%F %H:%M:%S"), received.op);
                println!("未知协议 {:#?}", received);
            }
        };
        Ok(())
    }
    pub async fn send(&mut self, operator: &QQBotOperation) -> AckermanResult<()> {
        self.wss.send(Message::Text(to_string(&operator)?)).await?;
        Ok(())
    }
    pub async fn send_heartbeat(&mut self) -> AckermanResult<()> {
        let protocol = QQBotOperation {
            op: 1,
            d: EventDispatcher::Integer(self.heartbeat_id),
            s: 0,
            t: "".to_string(),
            id: "".to_string(),
        };
        self.send(&protocol).await?;
        self.bot.on_heartbeat(self.heartbeat_id).await?;
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
                token: self.bot.build_bot_token(),
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
