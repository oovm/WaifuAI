use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageItem {
    pub id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub author: MessageAuthor,
    pub content: String,
    pub member: MessageMember,
    pub seq: i64,
    pub seq_in_channel: String,
    pub timestamp: String,
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
    pub id: String,
    pub username: String,
    pub bot: bool,
    pub avatar: String,
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
