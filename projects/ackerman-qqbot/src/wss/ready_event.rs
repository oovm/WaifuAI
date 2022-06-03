use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct User1 {
    pub id: String,
    pub username: String,
    pub bot: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyEvent {
    pub version: i64,
    pub session_id: String,
    pub user: User1,
    pub shard: Vec<i64>,
}
