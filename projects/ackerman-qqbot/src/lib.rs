pub use self::{
    errors::{AckermanError, AckermanResult},
    wss::QQBotWebsocket,
};

pub mod bots;
mod errors;
pub mod restful;
pub mod utils;
mod wss;

pub use self::restful::QQSecret;
