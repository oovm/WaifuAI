use std::{collections::BTreeMap, fs, path::PathBuf};

use async_trait::async_trait;
use qq_bot::{restful::SendMessageRequest, wss::MessageEvent, QQBotProtocol, QQResult, QQSecret, RequestBuilder, Url};
use tokio_tungstenite::tungstenite::http::Method;

pub use self::image_request::NovelAIRequest;

mod image_request;

pub struct AckermanQQBot {
    pub secret: QQSecret,
    pub here: PathBuf,
    pub cn_tags: BTreeMap<String, String>,
    pub users: DashMap<String, i64>,
}

impl AckermanQQBot {
    pub fn new(work_dir: PathBuf, secret: QQSecret) -> QQResult<Self> {
        let mut out = Self { secret, here: work_dir, cn_tags: BTreeMap::default(), users: Default::default() };
        out.ensure_path()?;
        for line in include_str!("dict.txt").lines() {
            if let Some((cn, en)) = line.split_once(",") {
                out.cn_tags.insert(cn.trim().to_string(), en.trim().to_string());
            }
        }
        Ok(out)
    }

    pub fn ensure_path(&self) -> QQResult {
        if !self.target_dir().exists() {
            fs::create_dir(self.target_dir())?
        }
        Ok(())
    }
    pub fn target_dir(&self) -> PathBuf {
        self.here.join("target/ackerman/")
    }
    async fn on_normal_message(&mut self, event: MessageEvent) -> QQResult {
        if !event.content.is_empty() {
            println!("    常规消息 {:#?}", event.content);
        }
        Ok(())
    }
    pub fn waifu_image_request(&mut self, rest: &str) -> QQResult<NovelAIRequest> {
        let mut image = NovelAIRequest::default();
        for tag in rest.split(|c| c == ',' || c == '，') {
            let tag = tag.trim().to_ascii_lowercase();
            match tag.as_str() {
                "横" | "w" | "landscape" => image.set_layout(2.0),
                "竖" | "h" | "portrait" => image.set_layout(0.5),
                "方" | "s" | "square" => image.set_layout(1.0),
                s if s.starts_with("质量") => {}
                s if s.starts_with("s") => {}
                s if s.starts_with("step") => {}
                s if s.starts_with("steps") => {}
                s if s.starts_with("步数") => {}
                _ => match self.cn_tags.get(&tag) {
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
    async fn on_message(&mut self, event: MessageEvent) -> QQResult {
        match event.content.as_str() {
            s if s.starts_with("waifu") => {
                let tags = self.waifu_image_request(&s["waifu".len()..s.len()])?;
                if !tags.is_empty() {
                    match event.attachments.first() {
                        None => {}
                        Some(s) => s.download(&self.target_dir()).await?,
                    }
                    let image_path = self.target_dir().join("{8DF6CF1E-304E-B9EA-E9D0-B6CBA8E4EBF6}.jpg");
                    let req = SendMessageRequest {
                        msg_id: event.id,
                        content: "".to_string(),
                        image_path,
                        file_image: "waifu".to_string(),
                        user_id: event.author.id,
                    };
                    req.send(self, event.channel_id).await?;
                }
                else {
                    println!("    waifu 空请求");
                }
                Ok(())
            }
            s if s.starts_with("furry") => {
                let tags = self.waifu_image_request(&s["furry".len()..s.len()])?;
                if !tags.is_empty() {
                    match event.attachments.first() {
                        None => {}
                        Some(s) => s.download(&self.target_dir()).await?,
                    }
                    let image_path = self.target_dir().join("{8DF6CF1E-304E-B9EA-E9D0-B6CBA8E4EBF6}.jpg");
                    let req = SendMessageRequest {
                        msg_id: event.id,
                        content: "".to_string(),
                        image_path,
                        file_image: "furry".to_string(),
                        user_id: event.author.id,
                    };
                    req.send(self, event.channel_id).await?;
                }
                else {
                    println!("    waifu 空请求");
                }
                Ok(())
            }
            // 不要处理 @开头的事件
            s if s.starts_with("<@!") => Ok(()),
            _ => self.on_normal_message(event).await,
        }
    }
}
