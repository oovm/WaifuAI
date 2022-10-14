use indexmap::IndexSet;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash, Hasher},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use serde_json::to_string;

pub mod image_request;

#[derive(Debug)]
pub struct ImageRequestBuilder {
    pub positive: IndexSet<String>,
    pub negative: IndexSet<String>,
    pub layout: ImageLayout,
    pub kind: NovelAIKind,
    pub image: Vec<u8>,
}

impl Default for ImageRequestBuilder {
    fn default() -> Self {
        let mut positive = IndexSet::default();
        let mut negative = IndexSet::default();
        for word in include_str!("positive.txt").split(',') {
            positive.insert(word.trim().to_lowercase());
        }
        for word in include_str!("negative.txt").split(',') {
            negative.insert(word.trim().to_lowercase());
        }
        Self { positive, negative, layout: ImageLayout::Portrait, kind: NovelAIKind::Anime, image: vec![] }
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
