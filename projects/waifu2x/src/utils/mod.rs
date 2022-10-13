use crate::Waifu2xResult;
use std::env::current_exe;
use tract_onnx::{
    onnx,
    prelude::{Framework, Graph, InferenceModelExt, RunnableModel, TypedFact, TypedOp},
};

pub type ModelType = RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

pub struct Waifu2x {
    model: Waifu2xModel,
}

pub enum Waifu2xModel {
    ESRGAN,
}

impl Waifu2x {
    pub fn render(&self) {}
}

pub fn find_model(name: &str) -> Waifu2xResult<ModelType> {
    let path = current_exe()?.with_file_name(name);
    let model = onnx().model_for_path(path)?.into_optimized()?.into_runnable()?;
    return Ok(model);
}
