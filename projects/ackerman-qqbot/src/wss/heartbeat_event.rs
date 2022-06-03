use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatEvent {
    pub heartbeat_interval: u64,
}
