use std::{
    fmt::{Debug, Formatter},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncWriteExt, net::TcpStream};
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
    pub async fn identify(&mut self) -> AckermanResult<Self> {
        let op = QQBotOperation {
            //
            op: 2,
            d: QQBotOperationDispatch { token: self.key.bot_token(), intents: 0, shard: vec![0] },
        };

        let (mut write, read) = self.wss.split();
        println!("sending");
        write.send(Message::Text(serde_json::to_string(&op)?)).await?;
        println!("sent");
        let read_future = read.for_each(|message| async {
            println!("receiving...");
            let data = message.unwrap().into_data();
            tokio::io::stdout().write(&data).await.unwrap();
            println!("received...");
        });
        read_future.await;

        todo!()
    }
}
