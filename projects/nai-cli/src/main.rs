use novel_ai::ImageRequestBuilder;

pub mod with_tags;


fn main() {
    let mut image = ImageRequestBuilder::default();
    image.build()
}