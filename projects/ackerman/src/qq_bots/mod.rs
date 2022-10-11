use std::{
    collections::BTreeMap,
    fs,
    fs::read_to_string,
    hash::{BuildHasher, Hash, Hasher},
    io::Write,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite::http::Method;

use qq_bot::{restful::SendMessageRequest, wss::MessageEvent, QQBotProtocol, QQResult, QQSecret, RequestBuilder, Url};

pub use self::image_request::NovelAIRequest;

mod image_request;

#[derive(Serialize, Deserialize)]
pub struct AckermanQQBot {
    #[serde(default)]
    pub config: AckermanConfig,
    #[serde(default)]
    pub here: PathBuf,
    #[serde(default)]
    pub cn_tags: BTreeMap<String, String>,
    #[serde(default)]
    pub users: DashMap<String, i64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AckermanConfig {
    qq: QQSecret,
    nai: NaiSecret,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NaiSecret {
    bearer: String,
}

impl AckermanConfig {
    pub fn load_toml(path: impl AsRef<Path>) -> QQResult<Self> {
        Ok(toml::from_str(&read_to_string(path)?)?)
    }
}

impl AckermanQQBot {
    pub fn loading() -> QQResult<Self> {
        let here = std::env::current_dir()?;
        let mut out = Self { config: Default::default(), here, cn_tags: BTreeMap::default(), users: Default::default() };
        if out.database_path().exists() {
            let db = read_to_string(out.database_path())?;
            let saved: Self = serde_json::from_str(&db)?;
            out.cn_tags = saved.cn_tags;
            out.users = saved.users;
        }
        out.ensure_path()?;
        out.load_dict();
        match AckermanConfig::load_toml("ackerman.toml") {
            Ok(o) => out.config = o,
            Err(_) => {}
        }
        Ok(out)
    }
    pub fn load_dict(&mut self) {
        for line in include_str!("dict.txt").lines() {
            if let Some((cn, en)) = line.split_once(",") {
                self.cn_tags.insert(cn.trim().to_string(), en.trim().to_string());
            }
        }
    }
    pub fn ensure_path(&self) -> QQResult {
        if !self.target_dir().exists() {
            fs::create_dir(self.target_dir())?
        }
        Ok(())
    }
    pub fn database_path(&self) -> PathBuf {
        self.here.join("target/ackerman/user.db")
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
        image.add_tag("best quality");
        image.add_tag("masterpiece");
        Ok(image)
    }
}

#[async_trait]
impl QQBotProtocol for AckermanQQBot {
    fn build_bot_token(&self) -> String {
        self.config.qq.bot_token()
    }
    fn build_request(&self, method: Method, url: Url) -> RequestBuilder {
        self.config.qq.as_request(method, url)
    }
    async fn on_message(&mut self, event: MessageEvent) -> QQResult {
        match event.content.as_str() {
            s if s.starts_with("waifu") => {
                let mut tags = self.waifu_image_request(&s["waifu".len()..s.len()])?;
                if !tags.is_empty() {
                    if let Some(s) = event.attachments.first() {
                        let image = s.download(&self.target_dir()).await?;
                        tags.set_reference_image(image)
                    }
                    let image_bytes = tags.nai_request(self).await?;
                    tags.nai_save(&self.target_dir(), &image_bytes).await?;
                    let req = SendMessageRequest {
                        msg_id: event.id, //
                        content: "".to_string(),
                        image_bytes,
                        user_id: event.author.id,
                    };
                    req.send(self, event.channel_id).await?;
                }
                else {
                    println!("    waifu 空请求");
                }
                Ok(())
            }
            s if s.starts_with("furry") => Ok(()),
            // 不要处理 @开头的事件
            s if s.starts_with("<@!") => Ok(()),
            _ => self.on_normal_message(event).await,
        }
    }
    async fn on_save(&mut self) -> QQResult {
        println!("    存档中");
        let s = serde_json::to_string_pretty(self)?;
        let mut save = fs::File::create(self.database_path())?;
        save.write_all(s.as_bytes())?;
        Ok(())
    }
}
