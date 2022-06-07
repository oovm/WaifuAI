use super::*;

use reqwest::Method;

use std::str::FromStr;

use url::Url;

/// `GET /users/@me/guilds`
///
/// <https://bot.q.qq.com/wiki/develop/api/openapi/user/guilds.html>
#[derive(Debug)]
pub struct GetGuildListResponse {
    pub items: Vec<GuildItem>,
}

impl GetGuildListResponse {
    pub fn end_point() -> String {
        if cfg!(debug_assertions) {
            format!("https://sandbox.api.sgroup.qq.com/users/@me/guilds")
        }
        else {
            format!("https://api.sgroup.qq.com/users/@me/guilds")
        }
    }
    pub async fn send(key: &QQSecret) -> AckermanResult<Self> {
        let url = Url::from_str(&Self::end_point())?;
        let response = key.as_request(Method::GET, url).send().await?;
        Ok(Self { items: response.json().await? })
    }
}

#[derive(Deserialize, Debug)]
pub struct GuildItem {
    /// 频道名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 频道头像地址
    #[serde(deserialize_with = "crate::utils::read_url")]
    pub icon: Url,
    /// 频道ID
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub id: u64,
    /// 	最大成员数
    pub max_members: u32,
    /// 成员数
    pub member_count: u32,
    /// 当前人是否是创建人
    pub owner: bool,
    /// 创建人用户ID
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub owner_id: u64,
    /// 加入时间
    pub joined_at: String,
}
