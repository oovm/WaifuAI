use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
    pub id: String,
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub guild_id: u64,
    #[serde(deserialize_with = "crate::utils::read_u64")]
    pub channel_id: u64,
    pub author: MessageAuthor,
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
    id: String,
    content_type: String,
    filename: String,
    height: u32,
    width: u32,
    size: u32,
    url: String,
}
