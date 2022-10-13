use image::ImageError;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub type Waifu2xResult<T = ()> = Result<T, Waifu2xError>;

#[derive(Debug)]
pub enum Waifu2xError {
    IOError(std::io::Error),
    ParseError(String),
    NetError(String),
    ImageError(ImageError),
    UnknownError(tract_onnx::tract_core::anyhow::Error),
}

impl Error for Waifu2xError {}

impl Display for Waifu2xError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Waifu2xError::IOError(e) => write!(f, "{}", e),
            Waifu2xError::ParseError(e) => write!(f, "{}", e),
            Waifu2xError::NetError(e) => write!(f, "{}", e),
            Waifu2xError::ImageError(e) => write!(f, "{}", e),
            Waifu2xError::UnknownError(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for Waifu2xError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
impl From<url::ParseError> for Waifu2xError {
    fn from(e: url::ParseError) -> Self {
        Self::ParseError(e.to_string())
    }
}
impl From<serde_json::Error> for Waifu2xError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(e.to_string())
    }
}

impl From<reqwest::Error> for Waifu2xError {
    fn from(e: reqwest::Error) -> Self {
        Self::NetError(e.to_string())
    }
}
impl From<ImageError> for Waifu2xError {
    fn from(e: ImageError) -> Self {
        Self::ImageError(e)
    }
}

impl From<tract_onnx::tract_core::anyhow::Error> for Waifu2xError {
    fn from(e: tract_onnx::tract_core::anyhow::Error) -> Self {
        Self::UnknownError(e)
    }
}
