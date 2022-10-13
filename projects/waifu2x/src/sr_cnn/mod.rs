use crate::{
    utils::{find_model, ModelType},
    Waifu2xResult,
};
use image::RgbImage;
use std::{env::current_dir, path::PathBuf, sync::LazyLock};
use tract_onnx::{
    prelude::*,
    tract_hir::{
        infer::{InferenceOp, ShapeFactoid},
        shapefactoid,
    },
};

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
const WH: (u32, u32) = (156, 156);
/// <https://github.com/tcyrus/waifu2x-onnx/tree/master/models/anime_style_art_rgb>
pub static WAIFU_SRCNN3: LazyLock<ModelType> = LazyLock::new(|| {
    let mut model = find_model("upconv_7_anime_style_art_rgb/scale2.0x_model.onnx").unwrap();
    match model.input_fact_mut(0) {
        Ok(order) => {
            println!("{:?}", order.shape);
            order.shape = shapefactoid![1, 3, 156, 156]
        }
        Err(_) => {}
    }
    model.into_optimized().unwrap().into_runnable().unwrap()
});

fn main(image: RgbImage) -> TractResult<()> {
    // https://github.com/TheFutureGadgetsLab/WaifuXL

    let resized = image::imageops::resize(&image, 156, 156, ::image::imageops::FilterType::Triangle);
    let image: Tensor =
        tract_ndarray::Array4::from_shape_fn((1, 3, 156, 156), |(_, c, y, x)| (resized[(x as _, y as _)][c] as f32 / 255.0))
            .into();
    let result = WAIFU_SRCNN3.run(tvec!(image))?;
    println!("result: {:?}", result);
    Ok(())
}
