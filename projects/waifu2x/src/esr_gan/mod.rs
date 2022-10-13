use crate::{
    utils::{find_model, ModelType},
    Waifu2xResult,
};
use image::RgbImage;
use std::{env::current_dir, path::PathBuf, sync::LazyLock};
use tract_onnx::prelude::*;

#[test]
fn test() -> Waifu2xResult {
    let path = PathBuf::from("senjougahara.png").canonicalize().unwrap();
    println!("{:?}", current_dir().unwrap().canonicalize().unwrap());
    println!("{:?}", path);
    println!("{}", path.exists());

    let image = image::open("senjougahara.png")?.to_rgb8();
    main(image)?;
    Ok(())
}

/// <https://github.com/TheFutureGadgetsLab/WaifuXL>
pub static WAIFU_ESRGAN: LazyLock<ModelType> = LazyLock::new(|| {
    // find_model("waifu-esrgan.onnx").unwrap()find_model("waifu-esrgan.onnx").unwrap()
    todo!()
});

fn main(image: RgbImage) -> TractResult<()> {
    // https://github.com/TheFutureGadgetsLab/WaifuXL

    let resized = image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        (resized[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    })
    .into();
    let result = WAIFU_ESRGAN.run(tvec!(image))?;
    let best = result[0].to_array_view::<f32>()?.iter().cloned().zip(2..).max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("result: {:?}", best);
    Ok(())
}
