use super::*;

impl QQSecret {
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
            // .bearer_auth(&self.bot_secret)
            .timeout(Duration::from_secs(3))
    }
    pub fn bot_token(&self) -> String {
        format!("Bot {}.{}", self.bot_app_id, self.bot_token)
    }

    pub fn bot_bearer(&self) -> String {
        self.bot_secret.to_string()
    }
}
