use std::sync::Arc;

use anyhow::Result;
use clip_img::Croper;
use clip_txt::Tokener;
use ndarray::{Array2, ArrayBase, Dim, IxDynImpl, OwnedRepr};
use ort::{
  environment::Environment, tensor::InputTensor, GraphOptimizationLevel, Session, SessionBuilder,
};

use crate::{img::ClipImg, providers::providers, txt::ClipTxt};

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

  pub fn run(
    &self,
    li: impl AsRef<[InputTensor]>,
  ) -> anyhow::Result<ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>> {
    let out = self.0.run(li.as_ref())?;
    Ok(out[0].try_extract::<f32>()?.view().to_owned())
  }
}
