#[derive(Debug)]
pub enum AckermanError {
    UnknownError,
    IOError(std::io::Error),
    NetError(reqwest::Error),
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
        Self::NetError(e)
    }
}
