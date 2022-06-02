pub use self::{
    errors::{AckermanError, AckermanResult},
    wss::QQBotWebsocket,
};

mod errors;
pub mod restful;
mod utils;
mod wss;

pub use self::restful::QQBotSecret;
