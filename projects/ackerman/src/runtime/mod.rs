use crate::{AckermanError, AckermanResult};
use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path, time::Duration};
use toml::from_str;

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
    guild_id: usize,
    channel_id: usize,
}

impl SecretKey {
    pub fn load(path: impl AsRef<Path>) -> AckermanResult<Self> {
        Ok(from_str(&read_to_string(path)?)?)
    }
    pub fn has_channel_id(&self) -> bool {
        if cfg!(debug_assertions) { self.test.channel_id != 0 } else { self.deploy.channel_id != 0 }
    }
    pub fn bot_token(&self) -> String {
        format!("Bot {}.{}", self.bot_app_id, self.bot_token)
    }
}

/// `GET /guilds/{guild_id}/channels`
/// <https://bot.q.qq.com/wiki/develop/api/openapi/guild/get_guild.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildResponse {
    code: i32,
    message: String,
}

impl GetGuildResponse {
    pub async fn send(key: &SecretKey) -> AckermanResult<Self> {
        let request_url = if cfg!(debug_assertions) {
            format!("https://sandbox.api.sgroup.qq.com/guilds/{guild_id}/channels", guild_id = key.test.guild_id)
        }
        else {
            format!("https://api.sgroup.qq.com/guilds/{guild_id}/channels", guild_id = key.deploy.guild_id)
        };
        println!("{}", request_url);
        let response = Client::default()
            .get(request_url)
            .header(USER_AGENT, "BotNodeSDK/v2.9.4")
            .header(AUTHORIZATION, key.bot_token())
            .timeout(Duration::from_secs(3))
            .send()
            .await?;
        let out: GetGuildResponse = response.json().await?;
        return Ok(out);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildItem {
    /// 子频道 id
    id: String,
    /// 频道 id
    guild_id: String,
    /// 子频道名
    name: String,
    /// 子频道类型 ChannelType
    r#type: u32,
    /// 子频道子类型 ChannelSubType
    sub_type: u32,
    /// 排序值，具体请参考 有关 position 的说明
    position: u32,
    /// 所属分组 id，仅对子频道有效，对 子频道分组（ChannelType=4） 无效
    parent_id: String,
    /// 创建人 id
    owner_id: String,
    /// 子频道私密类型 PrivateType
    private_type: u32,
    /// 子频道发言权限 SpeakPermission
    speak_permission: u32,
    /// 用于标识应用子频道应用类型，仅应用子频道时会使用该字段，具体定义请参考 应用子频道的应用类型
    application_id: String,
    /// 用户拥有的子频道权限 Permissions
    permissions: String,
}
