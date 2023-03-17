use std::{
    fs::File,
    io::Write,
    ops::{AddAssign, SubAssign},
};

use base64::{decode, encode};
use itertools::Itertools;
use reqwest::{header::CONTENT_TYPE, Client, Method};
use url::Url;

use crate::{NaiError, NaiResult};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageRequest {
    pub input: String,
    pub model: String,
    pub parameters: ImageParameters,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageParameters {
    pub width: u32,
    pub height: u32,
    pub scale: f32,
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

impl<T> AddAssign<T> for ImageRequestBuilder
where
    T: AsRef<str>,
{
    fn add_assign(&mut self, rhs: T) {
        for tag in rhs.as_ref().split(|f: char| [',', '，'].contains(&f)) {
            self.positive.insert(tag.trim().to_lowercase());
        }
    }
}

impl<T> SubAssign<T> for ImageRequestBuilder
where
    T: AsRef<str>,
{
    fn sub_assign(&mut self, rhs: T) {
        for tag in rhs.as_ref().split(|f: char| [',', '，'].contains(&f)) {
            self.negative.insert(tag.trim().to_lowercase());
        }
    }
}

impl ImageRequestBuilder {
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
        self.positive.is_empty()
    }
    pub fn nai_save(&self, dir: &PathBuf, bytes: &[u8]) -> NaiResult {
        let mut hasher = RandomState::default().build_hasher();
        bytes.hash(&mut hasher);
        let image_name = format!("{:0X}.png", hasher.finish());
        let image_path = dir.join(image_name);
        let mut file = File::create(&image_path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
    fn build_parameters(&self) -> ImageParameters {
        let no_ref_image = self.image.is_empty();
        let size = self.layout.small();
        let image = if no_ref_image { String::new() } else { encode(&self.image) };
        let steps = if no_ref_image { 28 } else { 50 };
        // 越小越遵守参考
        let strength = if no_ref_image { 0.9 } else { 0.6 };
        // 越大越遵守标签
        let scale = if no_ref_image { 11.0 } else { 13.0 };
        ImageParameters {
            width: size.0,
            height: size.1,
            quality_toggle: true,
            steps,
            image,
            scale,
            n_samples: 1,
            strength,
            sampler: "k_euler_ancestral".to_string(),
            seed: 42,
            uc: self.negative.iter().join(","),
            uc_preset: 0,
            noise: 0.10,
        }
    }
    pub fn build(&self) -> ImageRequest {
        let model = match self.kind {
            NovelAIKind::Anime => {
                if self.image.is_empty() {
                    "nai-diffusion"
                }
                else {
                    "safe-diffusion"
                }
            }
            NovelAIKind::Furry => "nai-diffusion-furry",
        };
        ImageRequest { input: self.positive.iter().join(","), model: model.to_string(), parameters: self.build_parameters() }
    }
}

impl ImageRequest {
    pub async fn request_image(&self, bearer: &str) -> NaiResult<Vec<u8>> {
        let nai_url = Url::from_str("https://api.novelai.net/ai/generate-image")?;
        let nai_request = Client::default()
            .request(Method::POST, nai_url)
            .header(CONTENT_TYPE, "application/json")
            // .header(USER_AGENT, "BotNodeSDK/v2.9.4")
            // .header("origin", "https://novelai.net")
            // .header("referer", "https://novelai.net/")
            .bearer_auth(bearer)
            .body(to_string(self)?)
            .timeout(Duration::from_secs(30));
        // text/event-stream
        let stream = nai_request.send().await?.text().await?;
        match stream.split_once("data:") {
            None => {}
            Some((_, image)) => match decode(image.trim()) {
                Ok(o) => return Ok(o),
                Err(e) => NaiError::NetError(e.to_string()),
            },
        }
        Err(NaiError::NetError(stream))
    }
}
