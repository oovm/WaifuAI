use crate::Waifu2xResult;
use std::env::{current_dir, current_exe};
use tract_onnx::{
    onnx,
    prelude::{Framework, Graph, InferenceModel, InferenceModelExt, RunnableModel, TypedFact, TypedOp},
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

pub fn find_model(name: &str) -> Waifu2xResult<InferenceModel> {
    let mut model_dir = current_exe()?;
    model_dir.pop();
    model_dir.push("models");
    model_dir.push(name);
    println!("Loading model from {:?}", model_dir);
    let model = onnx().model_for_path(model_dir)?;
    return Ok(model);
}
