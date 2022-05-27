#[derive(Debug)]
pub enum AckermanError {
    UnknownError,
    IOError(std::io::Error),
    NetError(String),
}

pub type AckermanResult<T = ()> = Result<T, AckermanError>;

impl From<std::io::Error> for AckermanError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<toml::de::Error> for AckermanError {
    fn from(_: toml::de::Error) -> Self {
        Self::UnknownError
    }
}

impl From<url::ParseError> for AckermanError {
    fn from(_: url::ParseError) -> Self {
        Self::UnknownError
    }
}

impl From<reqwest::Error> for AckermanError {
    fn from(e: reqwest::Error) -> Self {
        Self::NetError(e.to_string())
    }
}
impl From<tokio_tungstenite::tungstenite::Error> for AckermanError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::NetError(e.to_string())
    }
}

impl From<serde_json::Error> for AckermanError {
    fn from(e: serde_json::Error) -> Self {
        Self::NetError(e.to_string())
    }
}
