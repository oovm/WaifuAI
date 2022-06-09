use std::collections::BTreeMap;

use async_trait::async_trait;
use tokio_tungstenite::tungstenite::http::Method;

use qq_bot::{wss::MessageEvent, AckermanResult, QQBotProtocol, QQSecret, RequestBuilder, Url};

pub struct AckermanQQBot {
    pub secret: QQSecret,
}
const BAN_WORDS: &'static [&'static str] = &["nsfw"];

pub struct ImageRequest {
    tags: Vec<String>,
    aspect_ratio: f32,
}

impl Default for ImageRequest {
    fn default() -> Self {
        Self { tags: vec![], aspect_ratio: 0.0 }
    }
}

impl AckermanQQBot {
    async fn on_normal_message(&mut self, event: MessageEvent) -> AckermanResult {
        // event.content
        println!("收到消息 {:#?}", event);
        Ok(())
    }
    pub fn waifu_image_request(&mut self, rest: &str) -> AckermanResult<ImageRequest> {
        let mut image = ImageRequest::default();
        let mut dict = BTreeMap::default();
        for line in include_str!("dict.txt").lines() {
            match line.split_once(",") {
                None => {}
                Some((cn, en)) => dict.insert(cn.trim().to_string(), en.trim().to_string()),
            }
        }
        for tag in rest.split(|c| c == ',' || c == '，') {
            let tag = tag.trim().to_ascii_lowercase();
            if !BAN_WORDS.contains(&&*tag) {
                match tag.as_str() {
                    "横" | "w" => image.aspect_ratio = 2.0,
                    "竖" | "h" => image.aspect_ratio = 0.5,
                    "方" | "s" => image.aspect_ratio = 1.0,
                    _ => match dict.get(&tag) {
                        Some(normed) => {
                            if !normed.is_empty() {
                                image.tags.push(normed.to_string());
                            }
                        }
                        None => {
                            if tag.is_ascii() {
                                image.tags.push(tag);
                            }
                            else {
                                println!("未知 tag: {}", tag)
                            }
                        }
                    },
                }
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
                let image = self.waifu_image_request(&s["。waifu".len()..s.len()])?;
                Ok(())
            }
            _ => {
                println!("收到消息 {:#?}", event);
                Ok(())
            }
        }
    }
}
