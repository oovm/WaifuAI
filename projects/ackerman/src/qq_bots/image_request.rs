use std::{
    hash::{BuildHasher, Hash, Hasher},
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use serde_json::{to_string, Value};
use tokio::fs::File;
use tokio_tungstenite::tungstenite::http::{
    header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE},
    Method,
};

use qq_bot::{Client, QQBotProtocol, QQResult, Url};

use crate::qq_bots::AckermanQQBot;

#[derive(Debug, Hash)]
pub struct NovelAIRequest {
    tags: Vec<String>,
    layout: ImageLayout,
    kind: NovelAIKind,
}
#[derive(Debug, Hash)]
pub enum NovelAIKind {
    Anime = 0,
    Furry = 1,
}
#[derive(Debug, Hash)]
pub enum ImageLayout {
    Square = 0,
    Portrait = 1,
    Landscape = 2,
}

impl From<f32> for ImageLayout {
    fn from(v: f32) -> Self {
        if v > 1.0 {
            Self::Landscape
        }
        else if v < 1.0 {
            Self::Portrait
        }
        else {
            Self::Square
        }
    }
}

impl Default for NovelAIRequest {
    fn default() -> Self {
        Self { tags: vec![], layout: ImageLayout::Square, kind: NovelAIKind::Anime }
    }
}

impl NovelAIRequest {
    pub fn add_tag(&mut self, tag: &str) {
        if !tag.is_empty() {
            self.tags.push(tag.to_string())
        }
    }
    pub fn set_layout(&mut self, layout: impl Into<ImageLayout>) {
        self.layout = layout.into()
    }
    pub fn set_kind(&mut self, kind: impl Into<NovelAIKind>) {
        self.kind = kind.into()
    }
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
    pub fn cost(&self) -> i64 {
        let kind = match self.kind {
            NovelAIKind::Anime => 1.414,
            NovelAIKind::Furry => 1.0,
        };
        let cost = f32::log2(self.tags.len() as f32) * kind * 1000.0;
        cost.ceil() as i64
    }

    pub async fn nai_request(&self, bot: &AckermanQQBot) -> QQResult<Vec<u8>> {
        let nai_url = Url::from_str("https://api.novelai.net/ai/generate-image")?;
        let nai_request = Client::default()
            .request(Method::POST, nai_url)
            .header(CONTENT_TYPE, "application/json")
            // .header(USER_AGENT, "BotNodeSDK/v2.9.4")
            // .header("origin", "https://novelai.net")
            // .header("referer", "https://novelai.net/")
            .bearer_auth(&bot.config.nai.bearer)
            .body(to_string(&self.nai_request_body())?)
            .timeout(Duration::from_secs(10));
        // text/event-stream
        println!("    正在下载图片");
        let stream = nai_request.send().await?.text().await?;
        println!("    图片下载完成");
        println!("{}", stream);
        let image = &stream[27..stream.len()];
        let bytes = base64::decode(image).unwrap();
        Ok(bytes)
    }
    fn qq_content(&self) -> String {
        self.tags.join(",")
    }

    fn nai_request_body(&self) -> NaiRequest {
        let seed = 114514;
        let model = match self.kind {
            NovelAIKind::Anime => "nai-diffusion",
            NovelAIKind::Furry => "nai-diffusion-furry",
        };
        let width = match self.layout {
            ImageLayout::Square => 640,
            ImageLayout::Portrait => 512,
            ImageLayout::Landscape => 768,
        };
        let height = match self.layout {
            ImageLayout::Square => 640,
            ImageLayout::Portrait => 768,
            ImageLayout::Landscape => 512,
        };
        NaiRequest {
            input: self.tags.join(","),
            model: model.to_string(),
            parameters: Parameters {
                width,
                height,
                quality_toggle: true,
                steps: 28,
                scale: 11,
                n_samples: 1,
                sampler: "k_euler_ancestral".to_string(),
                seed,
                uc: "nsfw, bad anatomy".to_string(),
                uc_preset: 0,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct NaiRequest {
    pub input: String,
    pub model: String,
    pub parameters: Parameters,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub width: u64,
    pub height: u64,
    pub scale: u64,
    pub sampler: String,
    pub steps: u8,
    pub n_samples: u8,
    pub seed: u32,
    #[serde(rename = "ucPreset")]
    pub uc_preset: u64,
    #[serde(rename = "qualityToggle")]
    pub quality_toggle: bool,
    pub uc: String,
}
