use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{
    collections::{hash_map::RandomState},
    hash::{BuildHasher, Hash, Hasher},
    io::Write,
    path::{PathBuf},
    str::FromStr,
    time::Duration,
};
use tokio::{fs::File, io::AsyncWriteExt};

mod image_request;

#[derive(Debug, Hash)]
pub struct ImageRequestBuilder {
    tags: Vec<String>,
    layout: ImageLayout,
    kind: NovelAIKind,
    image: Vec<u8>,
}

impl Default for ImageRequestBuilder {
    fn default() -> Self {
        Self { tags: vec![], layout: ImageLayout::Portrait, kind: NovelAIKind::Anime, image: vec![] }
    }
}

#[derive(Debug, Hash)]
pub enum NovelAIKind {
    Anime = 0,
    Furry = 1,
}

#[derive(Debug, Hash)]
pub enum ImageLayout {
    Square = 0,
    Portrait = 1,
    Landscape = 2,
}

