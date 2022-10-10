#[derive(Debug)]
pub struct NovelAIRequest {
    tags: Vec<String>,
    pub aspect_ratio: f32,
}

impl Default for NovelAIRequest {
    fn default() -> Self {
        Self { tags: vec![], aspect_ratio: 0.0 }
    }
}

impl NovelAIRequest {
    pub fn add_tag(&mut self, tag: &str) {
        if !tag.is_empty() {
            self.tags.push(tag.to_string())
        }
    }
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}
