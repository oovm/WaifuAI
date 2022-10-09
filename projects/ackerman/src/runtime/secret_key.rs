use super::*;
use reqwest::{Method, RequestBuilder};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretKey {
    bot_app_id: u64,
    bot_secret: String,
    bot_token: String,
    test: ChannelIds,
    deploy: ChannelIds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelIds {
    guild_id: u64,
    channel_id: u64,
}

impl SecretKey {
    pub fn load(path: impl AsRef<Path>) -> AckermanResult<Self> {
        Ok(from_str(&read_to_string(path)?)?)
    }
    pub fn channel_id(&self) -> u64 {
        if cfg!(debug_assertions) { self.test.channel_id } else { self.deploy.channel_id }
    }
    pub fn guild_id(&self) -> u64 {
        if cfg!(debug_assertions) { self.test.guild_id } else { self.deploy.guild_id }
    }
    pub fn as_request(&self, method: Method, url: Url) -> RequestBuilder {
        Client::default()
            .request(method, url)
            .header(USER_AGENT, "BotNodeSDK/v2.9.4")
            .header(AUTHORIZATION, self.bot_token())
            .timeout(Duration::from_secs(3))
    }
    pub fn bot_token(&self) -> String {
        format!("Bot {}.{}", self.bot_app_id, self.bot_token)
    }
}
