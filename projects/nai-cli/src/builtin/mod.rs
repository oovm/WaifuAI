use std::{collections::BTreeMap, sync::LazyLock};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub static BUILTIN_PROMPTS: LazyLock<Prompts> = LazyLock::new(|| Prompts::default());

#[derive(Serialize, Deserialize)]
pub struct Prompts {
    normal: BTreeMap<String, String>,
    ss: BTreeMap<String, String>,
}

impl Default for Prompts {
    fn default() -> Self {
        toml::from_str(include_str!("builtin.toml")).unwrap()
    }
}

impl Prompts {
    pub fn normal(&self) -> (String, String) {
        let rng = &mut thread_rng();
        let idx = rng.gen_range(0..self.normal.len());
        let (k, v) = self.normal.iter().nth(idx).unwrap();
        (k.to_string(), v.to_string())
    }
    pub fn se_se(&self) -> (String, String) {
        const DEFAULT_NAME: &'static str = "miku";
        const DEFAULT_TAGS: &'static str = "best quality, masterpiece, highres, original, extremely detailed wallpaper";

        let mut map = BTreeMap::default();
        map.insert(DEFAULT_NAME, DEFAULT_TAGS);

        let rng = &mut thread_rng();
        let idx = rng.gen_range(0..map.len());
        match map.iter().nth(idx) {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (DEFAULT_NAME.to_string(), DEFAULT_TAGS.to_string()),
        }
    }
}
