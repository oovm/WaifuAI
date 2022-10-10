use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginEvent {
    pub version: i64,
    pub session_id: String,
    pub user: User,
    pub shard: Vec<i64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub bot: bool,
}
