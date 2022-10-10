use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectEvent {
    pub token: String,
    pub intents: u32,
    pub shard: Vec<u32>,
}
