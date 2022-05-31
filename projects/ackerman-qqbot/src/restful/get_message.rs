use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageItem {
    pub author: Author,
    pub channel_id: String,
    pub content: String,
    pub guild_id: String,
    pub id: String,
    pub member: Member,
    pub timestamp: String,
    pub seq: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub joined_at: String,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub avatar: String,
    pub bot: bool,
    pub id: String,
    pub username: String,
}
