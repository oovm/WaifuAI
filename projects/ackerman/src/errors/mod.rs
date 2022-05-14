
#[derive(Debug)]
pub enum AckermanError {
    UnknownError,
    IOError(std::io::Error)
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