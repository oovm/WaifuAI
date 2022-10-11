use std::{path::PathBuf, time::Duration};

use reqwest::{header::USER_AGENT, Client};
use tokio::{fs::File, io::AsyncWriteExt};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
    pub id: String,
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub guild_id: u64,
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub channel_id: u64,
    pub author: MessageAuthor,
    #[serde(default)]
    pub content: String,
    pub member: MessageMember,
    pub seq: i64,
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub seq_in_channel: u64,
    pub timestamp: String,
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageMember {
    pub nick: String,
    pub joined_at: String,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageAuthor {
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub id: u64,
    pub username: String,
    pub avatar: String,
    pub bot: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageAttachment {
    pub id: String,
    pub content_type: String,
    pub filename: String,
    pub height: u32,
    pub width: u32,
    pub size: u32,
    pub url: String,
}

impl MessageAttachment {
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    pub async fn download(&self, dir: &PathBuf) -> QQResult<Vec<u8>> {
        let url = Url::from_str(&format!("https://{}", self.url))?;
        let request = Client::default()
            .request(Method::GET, url)
            .header(USER_AGENT, "BotNodeSDK/v2.9.4")
            .timeout(Duration::from_secs(30));
        let bytes = request.send().await?.bytes().await?;
        let path = dir.join(&self.filename);
        let mut file = File::create(path).await?;
        file.write_all(&bytes).await?;
        Ok(bytes.to_vec())
    }
}
