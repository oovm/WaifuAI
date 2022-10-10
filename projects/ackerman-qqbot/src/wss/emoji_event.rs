use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmojiEvent {
    pub channel_id: String,
    pub emoji: Emoji,
    pub guild_id: String,
    pub user_id: String,
    pub target: EmojiTarget,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    pub id: String,
    pub r#type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmojiTarget {
    pub id: String,
    pub r#type: u32,
}
