use super::*;

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
    pub fn has_channel_id(&self) -> bool {
        if cfg!(debug_assertions) { self.test.channel_id != 0 } else { self.deploy.channel_id != 0 }
    }
    pub fn guild_id(&self) -> u64 {
        if cfg!(debug_assertions) { self.test.guild_id } else { self.deploy.guild_id }
    }

    pub fn bot_token(&self) -> String {
        format!("Bot {}.{}", self.bot_app_id, self.bot_token)
    }
}
