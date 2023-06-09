use std::sync::Arc;

use anyhow::Result;
use ndarray::Ix2;
use ort::{
  environment::Environment, tensor::InputTensor, GraphOptimizationLevel, Session, SessionBuilder,
};

use crate::typedef::Arr;

pub struct ClipSession(Session);

impl ClipSession {
  pub fn new(dir: &str, env: &Arc<Environment>, onnx: &str) -> Result<Self> {
    Ok(Self(
      SessionBuilder::new(env)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_inter_threads(num_cpus::get() as i16)?
        .with_model_from_file(format!("{}/{}.onnx", dir, onnx))?,
    ))
  }

  pub fn run(&self, li: impl AsRef<[InputTensor]>) -> anyhow::Result<Arr> {
    let out = self.0.run(li.as_ref())?;
    Ok(
      out[0]
        .try_extract::<f32>()?
        .view()
        .to_owned()
        .into_dimensionality::<Ix2>()?,
    )
  }
}
