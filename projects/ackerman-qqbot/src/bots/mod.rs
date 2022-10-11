use std::{fs::read_to_string, path::Path, time::Duration};

use async_trait::async_trait;
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    Utc,
};
use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Client, Method, RequestBuilder,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    wss::{EmojiEvent, HeartbeatEvent, LoginEvent, MessageEvent},
    QQResult,
};

mod secret_key;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct QQSecret {
    bot_app_id: u64,
    bot_secret: String,
    bot_token: String,
    test: ChannelIds,
    deploy: ChannelIds,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ChannelIds {
    guild_id: u64,
    channel_id: u64,
}

fn current_time<'a>() -> DelayedFormat<StrftimeItems<'a>> {
    Utc::now().format("%F %H:%M:%S")
}

#[async_trait]
#[allow(unused_variables)]
pub trait QQBotProtocol: Send {
    fn build_bot_token(&self) -> String;
    fn build_request(&self, method: Method, url: Url) -> RequestBuilder;
    async fn on_connected(&mut self, event: HeartbeatEvent) -> QQResult {
        println!("[{}] 协议 10", current_time());
        println!("    已连接");
        Ok(())
    }
    async fn on_login_success(&mut self, event: LoginEvent) -> QQResult {
        println!("[{}] 协议 9", current_time());
        println!("    登录成功, 登陆为 {:?}", event.user.username);
        Ok(())
    }
    async fn on_login_failure(&mut self) -> QQResult {
        println!("[{}] 协议 9", current_time());
        println!("    鉴权参数有误");
        Ok(())
    }
    async fn on_heartbeat(&mut self, heartbeat_id: u32) -> QQResult {
        println!("[{}] 协议 1", current_time());
        println!("    发送心跳包 {}", heartbeat_id);
        Ok(())
    }
    async fn on_message(&mut self, event: MessageEvent) -> QQResult {
        println!("[{}] 协议 0", current_time());
        println!("    收到消息, 发送者为 {:?}", event.author.username);
        Ok(())
    }
    async fn on_emoji(&mut self, event: EmojiEvent) -> QQResult {
        println!("[{}] 协议 0", current_time());
        println!("    消息 {} 表情变动", event.target.id);
        Ok(())
    }
    async fn on_save(&mut self) -> QQResult {
        Ok(())
    }
}

pub struct SimpleBot {
    pub heartbeat_interval: u64,
    pub secret: QQSecret,
}

#[async_trait]
impl QQBotProtocol for SimpleBot {
    fn build_bot_token(&self) -> String {
        self.secret.bot_token()
    }
    fn build_request(&self, method: Method, url: Url) -> RequestBuilder {
        self.secret.as_request(method, url)
    }
    async fn on_connected(&mut self, event: HeartbeatEvent) -> QQResult {
        self.heartbeat_interval = event.heartbeat_interval;
        Ok(())
    }
    async fn on_message(&mut self, event: MessageEvent) -> QQResult {
        // event.content
        println!("收到消息 {:#?}", event);
        Ok(())
    }
}
