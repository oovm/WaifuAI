use std::collections::BTreeMap;

use rand::{thread_rng, Rng};

pub struct BuiltinPrompt {}

impl BuiltinPrompt {
    pub fn normal() -> (String, String) {
        const DEFAULT_NAME: &'static str = "miku";
        const DEFAULT_TAGS: &'static str = "best quality, masterpiece, highres, original, extremely detailed wallpaper, miku";

        let mut map = BTreeMap::default();
        map.insert(DEFAULT_NAME, DEFAULT_TAGS);

        let rng = &mut thread_rng();
        let idx = rng.gen_range(0..map.len());
        match map.iter().nth(idx) {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (DEFAULT_NAME.to_string(), DEFAULT_TAGS.to_string()),
        }
    }
    pub fn se_se() -> (String, String) {
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
