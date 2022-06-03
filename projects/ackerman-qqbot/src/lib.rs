pub use self::{
    errors::{AckermanError, AckermanResult},
    wss::QQBotWebsocket,
};

mod errors;
pub mod restful;
pub mod utils;
mod wss;

pub use self::restful::QQBotSecret;
