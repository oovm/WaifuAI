use image::RgbImage;
use std::{
    env::{current_dir, current_exe},
    sync::LazyLock,
};
use tract_onnx::prelude::*;

#[test]
fn test() {
    let image = image::open("senjougahara.png").unwrap().to_rgb8();
    main(image).unwrap()
}

pub struct Waifu2x {
    model: Waifu2xModel,
}

pub enum Waifu2xModel {
    ESRGAN,
}

impl Waifu2x {
    pub fn render(&self) {}
}

pub static WAIFU_ESRGAN: LazyLock<RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>> =
    LazyLock::new(|| onnx().model_for_path("waifu-esrgan.onnx")?.into_optimized()?.into_runnable()?);

fn find_file(name: &str) {
    current_exe().unwrap().with_file_name("waifu-esrgan.onnx")
}

fn main(image: RgbImage) -> TractResult<()> {
    // https://github.com/TheFutureGadgetsLab/WaifuXL
    let model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>> =
        onnx().model_for_path("waifu-esrgan.onnx")?.into_optimized()?.into_runnable()?;

    let resized = image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        (resized[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    })
    .into();
    let result = model.run(tvec!(image))?;
    let best = result[0].to_array_view::<f32>()?.iter().cloned().zip(2..).max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("result: {:?}", best);
    Ok(())
}
