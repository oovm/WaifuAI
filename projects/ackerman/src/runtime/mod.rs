use crate::{AckermanError, AckermanResult};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path};
use toml::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretKey {
    pub bot_app_id: u64,
    pub bot_secret: String,
    pub bot_token: String,
    pub guild_id: u64,
    pub test_channel: u64,
}

impl SecretKey {
    pub fn load(path: impl AsRef<Path>) -> AckermanResult<Self> {
        Ok(from_str(&read_to_string(path)?)?)
    }
}

/// `GET /guilds/{guild_id}/channels`
/// <https://bot.q.qq.com/wiki/develop/api/openapi/guild/get_guild.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildResponse {
    code: i32,
    message: String,
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
