use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

use async_trait::async_trait;
use formdata::{FilePart, FormData};
use hyper::header::Headers;
use qq_bot::{
    restful::SendMessageRequest,
    wss::{MessageAttachment, MessageEvent},
    AckermanResult, QQBotProtocol, QQSecret, RequestBuilder, Url,
};
use tokio_tungstenite::tungstenite::http::Method;

pub use self::image_request::NovelAIRequest;

mod image_request;

pub struct AckermanQQBot {
    pub secret: QQSecret,
    pub here: PathBuf,
}

impl AckermanQQBot {
    async fn on_normal_message(&mut self, event: MessageEvent) -> AckermanResult {
        println!("    常规消息 {:#?}", event.content);
        Ok(())
    }
    pub fn waifu_image_request(&mut self, rest: &str) -> AckermanResult<NovelAIRequest> {
        let mut image = NovelAIRequest::default();
        let mut dict = BTreeMap::default();
        for line in include_str!("dict.txt").lines() {
            if let Some((cn, en)) = line.split_once(",") {
                dict.insert(cn.trim().to_string(), en.trim().to_string());
            }
        }
        for tag in rest.split(|c| c == ',' || c == '，') {
            let tag = tag.trim().to_ascii_lowercase();
            match tag.as_str() {
                "横" | "w" | "portrait" => image.aspect_ratio = 2.0,
                "竖" | "h" | "landscape" => image.aspect_ratio = 0.5,
                "方" | "s" | "square" => image.aspect_ratio = 1.0,
                s if s.starts_with("质量") => {}
                s if s.starts_with("s") => {}
                s if s.starts_with("step") => {}
                s if s.starts_with("steps") => {}
                s if s.starts_with("步数") => {}
                _ => match dict.get(&tag) {
                    Some(normed) => {
                        if !normed.is_empty() {
                            image.add_tag(normed);
                        }
                    }
                    None => {
                        if tag.is_ascii() {
                            image.add_tag(&tag);
                        }
                        else {
                            println!("未知 tag: {}", tag)
                        }
                    }
                },
            }
        }
        Ok(image)
    }
}

#[async_trait]
impl QQBotProtocol for AckermanQQBot {
    fn build_bot_token(&self) -> String {
        self.secret.bot_token()
    }
    fn build_request(&self, method: Method, url: Url) -> RequestBuilder {
        self.secret.as_request(method, url)
    }
    async fn on_message(&mut self, event: MessageEvent) -> AckermanResult {
        match event.content.as_str() {
            s if s.starts_with("\\waifu") => {
                let image = self.waifu_image_request(&s["\\waifu".len()..s.len()])?;

                Ok(())
            }
            s if s.starts_with(".waifu") => {
                let image = self.waifu_image_request(&s[".waifu".len()..s.len()])?;
                Ok(())
            }
            s if s.starts_with("。waifu") => {
                let tags = self.waifu_image_request(&s["。waifu".len()..s.len()])?;
                if !tags.is_empty() {
                    match event.attachments.first() {
                        None => {}
                        Some(s) => {
                            let path = PathBuf::from("target/tmp");
                            s.download(&path).await?
                        }
                    }
                    println!("{:?}", std::env::current_dir().unwrap());
                    let image_path = PathBuf::from_str("/target/tmp/{8DF6CF1E-304E-B9EA-E9D0-B6CBA8E4EBF6}.jpg..jpeg").unwrap();
                    println!("{}", image_path.exists());
                    let req =
                        SendMessageRequest { msg_id: event.id, content: format!("{:#?}", tags), file_image: Some(image_path) };
                    req.send(self, event.channel_id, event.author.id).await?;
                }
                else {
                    println!("    waifu 空请求");
                }
                Ok(())
            }
            _ => self.on_normal_message(event).await,
        }
    }
}
