#[derive(Debug)]
pub enum QQError {
    UnknownError,
    IOError(std::io::Error),
    NetError(String),
}

pub type AckermanResult<T = ()> = Result<T, QQError>;

impl From<std::io::Error> for QQError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<toml::de::Error> for QQError {
    fn from(_: toml::de::Error) -> Self {
        Self::UnknownError
    }
}

impl From<url::ParseError> for QQError {
    fn from(_: url::ParseError) -> Self {
        Self::UnknownError
    }
}

impl From<reqwest::Error> for QQError {
    fn from(e: reqwest::Error) -> Self {
        Self::NetError(e.to_string())
    }
}
impl From<tokio_tungstenite::tungstenite::Error> for QQError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::NetError(e.to_string())
    }
}

impl From<serde_json::Error> for QQError {
    fn from(e: serde_json::Error) -> Self {
        Self::NetError(e.to_string())
    }
}
