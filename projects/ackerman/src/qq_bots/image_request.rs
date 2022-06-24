use crate::qq_bots::AckermanQQBot;
use qq_bot::{QQBotProtocol, QQResult, Url};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Value};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    str::FromStr,
};
use tokio_tungstenite::tungstenite::http::Method;

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
    pub async fn nai_request(&self, bot: &AckermanQQBot) -> QQResult<Value> {
        let nai_url = Url::from_str("https://api.novelai.net/ai/generate-image")?;
        let nai_request = bot
            .build_request(Method::POST, nai_url)
            .bearer_auth(&bot.config.nai.bearer)
            .body(to_string(&self.nai_request_body())?)
            .send()
            .await?;
        Ok(nai_request.json().await?)
    }
    fn qq_content(&self) -> String {
        self.tags.join(",")
    }

    fn nai_request_body(&self) -> NaiRequest {
        let s = RandomState::new();
        let mut hasher = s.build_hasher();
        self.hash(&mut hasher);
        let seed = hasher.finish();
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
                scale: 13,
                sampler: "k_euler_ancestral".to_string(),
                steps: 28,
                seed,
                n_samples: 1,
                quality_toggle: true,
                uc_preset: 0,
                uc: "".to_string(),
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
    pub steps: i64,
    pub seed: u64,
    pub n_samples: i64,
    #[serde(rename = "ucPreset")]
    pub uc_preset: i64,
    #[serde(rename = "qualityToggle")]
    pub quality_toggle: bool,
    pub uc: String,
}
