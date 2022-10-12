use std::{
    error::Error,
    fmt::{Display, Formatter},
    num::{ParseFloatError, ParseIntError},
};

pub type NaiResult<T = ()> = Result<T, NaiError>;

#[derive(Debug)]
pub enum NaiError {
    IOError(std::io::Error),
    ParseError(String),
    NetError(String),
}

impl Error for NaiError {}

impl Display for NaiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NaiError::IOError(e) => write!(f, "{}", e),
            NaiError::ParseError(e) => write!(f, "{}", e),
            NaiError::NetError(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for NaiError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
impl From<url::ParseError> for NaiError {
    fn from(e: url::ParseError) -> Self {
        Self::ParseError(e.to_string())
    }
}

impl From<serde_json::Error> for NaiError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(e.to_string())
    }
}
impl From<ParseIntError> for NaiError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseError(e.to_string())
    }
}
impl From<ParseFloatError> for NaiError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseError(e.to_string())
    }
}

impl From<reqwest::Error> for NaiError {
    fn from(e: reqwest::Error) -> Self {
        Self::NetError(e.to_string())
    }
}
