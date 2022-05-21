use super::*;
use reqwest::Method;
use std::str::FromStr;
use url::Url;

/// `GET /guilds/{guild_id}/channels`
///
/// <https://bot.q.qq.com/wiki/develop/api/openapi/guild/get_guild.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChannelListResponse {
    pub items: Vec<ChannelItem>,
}

impl GetChannelListResponse {
    pub fn end_point(key: &SecretKey) -> String {
        if cfg!(debug_assertions) {
            format!("https://sandbox.api.sgroup.qq.com/guilds/{guild_id}/channels", guild_id = key.guild_id())
        }
        else {
            format!("https://api.sgroup.qq.com/guilds/{guild_id}/channels", guild_id = key.guild_id())
        }
    }

    pub async fn send(key: &SecretKey) -> AckermanResult<Self> {
        let url = Url::from_str(&Self::end_point(key))?;
        let response = key.as_request(Method::GET, url).send().await?;
        let out: Vec<ChannelItem> = response.json().await?;
        return Ok(Self { items: out });
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelItem {
    /// 子频道 id
    pub id: String,
    /// 频道 id
    pub guild_id: String,
    /// 子频道名
    pub name: String,
    /// 子频道类型 ChannelType
    pub r#type: u32,
    /// 子频道子类型 ChannelSubType
    pub sub_type: u32,
    /// 排序值，具体请参考 有关 position 的说明
    pub position: u32,
    /// 所属分组 id，仅对子频道有效，对 子频道分组（ChannelType=4） 无效
    pub parent_id: String,
    /// 创建人 id
    pub owner_id: String,
    /// 子频道私密类型 PrivateType
    pub private_type: Option<u32>,
    /// 子频道发言权限 SpeakPermission
    pub speak_permission: Option<u32>,
    /// 用于标识应用子频道应用类型，仅应用子频道时会使用该字段，具体定义请参考 应用子频道的应用类型
    pub application_id: Option<String>,
    /// 用户拥有的子频道权限 Permissions
    pub permissions: Option<String>,
}
