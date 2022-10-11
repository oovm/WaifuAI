use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectEvent {
    pub token: String,
    pub intents: u32,
    pub shard: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct QQBotConnected {
    pub shards: u32,
    pub url: String,
    pub session_start_limit: SessionStartLimit,
}

#[derive(Deserialize, Debug)]
pub struct SessionStartLimit {
    pub max_concurrency: u32,
    pub remaining: u32,
    pub reset_after: u32,
    pub total: u32,
}
