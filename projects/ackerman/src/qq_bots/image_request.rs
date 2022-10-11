#[derive(Debug)]
pub struct NovelAIRequest {
    tags: Vec<String>,
    layout: ImageLayout,
    kind: NovelAIKind,
}
#[derive(Debug)]
pub enum NovelAIKind {
    Anime = 0,
}
#[derive(Debug)]
pub enum ImageLayout {
    Square = 0,
    Portrait = 1,
    Landscape = 2,
}

impl From<f32> for ImageLayout {
    fn from(v: f32) -> Self {
        if v > 1.0 {
            Self::Landscape
        }
        else if v < 1.0 {
            Self::Portrait
        }
        else {
            Self::Square
        }
    }
}

impl Default for NovelAIRequest {
    fn default() -> Self {
        Self { tags: vec![], layout: ImageLayout::Square, kind: NovelAIKind::Anime }
    }
}

impl NovelAIRequest {
    pub fn add_tag(&mut self, tag: &str) {
        if !tag.is_empty() {
            self.tags.push(tag.to_string())
        }
    }
    pub fn set_layout(&mut self, layout: impl Into<ImageLayout>) {
        self.layout = layout.into()
    }
    pub fn set_kind(&mut self, kind: impl Into<NovelAIKind>) {
        self.kind = kind.into()
    }
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
    pub fn cost(&self) -> i64 {
        let kind = match self.kind {
            NovelAIKind::Anime => 1.414,
        };
        let cost = f32::log2(self.tags.len() as f32) * kind * 1000.0;
        cost.ceil() as i64
    }
}
