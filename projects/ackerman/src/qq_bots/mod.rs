use std::collections::BTreeMap;

use async_trait::async_trait;
use tokio_tungstenite::tungstenite::http::Method;

use qq_bot::{
    restful::SendMessageRequest,
    wss::{MessageAttachment, MessageEvent},
    AckermanResult, QQBotProtocol, QQSecret, RequestBuilder, Url,
};

pub use self::image_request::NovelAIRequest;

mod image_request;

pub struct AckermanQQBot {
    pub secret: QQSecret,
}

impl AckermanQQBot {
    async fn on_normal_message(&mut self, event: MessageEvent) -> AckermanResult {
        // event.content
        println!("收到消息 {:#?}", event);
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
                    println!("{event:#?}");
                    match event.attachments.first() {
                        None => {}
                        Some(s) => s.url,
                    }

                    let req = SendMessageRequest {
                        msg_id: event.id,
                        content: format!("{:#?}", tags),
                        image: event.attachments.first().cloned(),
                    };
                    req.send(self, event.channel_id, event.author.id).await?;
                }
                else {
                    println!("    waifu 空请求");
                }
                Ok(())
            }
            _ => {
                println!("    常规消息 {:#?}", event.content);
                Ok(())
            }
        }
    }
}
