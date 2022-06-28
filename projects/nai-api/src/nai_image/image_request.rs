use base64::decode;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method};
use url::Url;
use crate::{NaiError, NaiResult};
use super::*;

#[derive(Serialize, Deserialize)]
struct Request {
    pub input: String,
    pub model: String,
    pub parameters: Parameters,
}

#[derive(Serialize, Deserialize)]
struct Parameters {
    pub width: u64,
    pub height: u64,
    pub scale: u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub image: String,
    pub sampler: String,
    pub steps: u8,
    pub n_samples: u8,
    pub strength: f32,
    pub noise: f32,
    pub seed: u32,
    #[serde(rename = "ucPreset")]
    pub uc_preset: u64,
    #[serde(rename = "qualityToggle")]
    pub quality_toggle: bool,
    pub uc: String,
}


impl From<f32> for ImageLayout {
    fn from(v: f32) -> Self {
        if v > 1.05 {
            Self::Landscape
        }
        else if v < 0.95 {
            Self::Portrait
        }
        else {
            Self::Square
        }
    }
}

impl ImageRequest {
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
    pub fn set_reference_image(&mut self, image: Vec<u8>) {
        self.image = image;
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
    pub async fn nai_save(&self, dir: &PathBuf, bytes: &[u8]) -> NaiResult {
        let mut hasher = RandomState::default().build_hasher();
        bytes.hash(&mut hasher);
        let image_name = format!("{:0X}.png", hasher.finish());
        let image_path = dir.join(image_name);
        let mut file = File::create(&image_path).await?;
        file.write_all(&bytes).await?;
        Ok(())
    }
    pub async fn nai_request(&self, bearer: &str) -> NaiResult<Vec<u8>> {
        let nai_url = Url::from_str("https://api.novelai.net/ai/generate-image")?;
        let nai_request = Client::default()
            .request(Method::POST, nai_url)
            .header(CONTENT_TYPE, "application/json")
            // .header(USER_AGENT, "BotNodeSDK/v2.9.4")
            // .header("origin", "https://novelai.net")
            // .header("referer", "https://novelai.net/")
            .bearer_auth(bearer)
            .body(to_string(&self.nai_request_body())?)
            .timeout(Duration::from_secs(10));
        // text/event-stream
        let stream = nai_request.send().await?.text().await?;
        match stream.split_once("data:") {
            None => {}
            Some((_, image)) => match decode(image.trim()) {
                Ok(o) => return Ok(o),
                Err(_) => {}
            },
        }
        Err(NaiError::NetError(stream))
    }
    fn qq_content(&self) -> String {
        self.tags.join(",")
    }

    fn nai_request_body(&self) -> Request {
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen();
        let no_ref_image = self.image.is_empty();
        let model = match self.kind {
            NovelAIKind::Anime => {
                if no_ref_image {
                    "nai-diffusion"
                }
                else {
                    "safe-diffusion"
                }
            }
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
        let image = if no_ref_image { String::new() } else { base64::encode(&self.image) };
        let steps = if no_ref_image { 28 } else { 50 };
        // 越小越遵守参考
        let strength = if no_ref_image { 0.9 } else { 0.6 };
        // 越大越遵守标签
        let scale = if no_ref_image { 11 } else { 13 };
        Request {
            input: self.tags.join(","),
            model: model.to_string(),
            parameters: Parameters {
                width,
                height,
                quality_toggle: true,
                steps,
                image,
                scale,
                n_samples: 1,
                strength,
                sampler: "k_euler_ancestral".to_string(),
                seed,
                uc: "nsfw, bad anatomy".to_string(),
                uc_preset: 0,
                noise: 0.00,
            },
        }
    }
}

