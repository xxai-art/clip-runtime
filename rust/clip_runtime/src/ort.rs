use std::{path::Path, sync::Arc};

use anyhow::Result;
use clip_txt::Tokener;
use ort::{environment::Environment, GraphOptimizationLevel, SessionBuilder};

use crate::{img::ClipImg, providers::providers, txt::ClipTxt};

pub struct ClipOrt {
  pub env: Arc<Environment>,
}

impl ClipOrt {
  pub fn new() -> Result<Self> {
    Ok(ClipOrt {
      env: Arc::new(
        Environment::builder()
          .with_name("clip")
          .with_execution_providers(providers())
          .build()?,
      ),
    })
  }

  // pub fn img(&self) -> ClipImg {}

  pub fn txt(
    &self,
    dir: impl AsRef<Path>,
    onnx: &str,
    context_length: usize,
  ) -> clip_txt::Result<ClipTxt> {
    let env = self.env.clone();
    let dir = dir.as_ref().display();
    let sess = SessionBuilder::new(&env)?
      .with_optimization_level(GraphOptimizationLevel::Level3)?
      .with_inter_threads(num_cpus::get() as i16)?
      .with_model_from_file(format!("{}/{}.onnx", dir, onnx))?;
    Ok(ClipTxt {
      env,
      sess,
      tokener: Tokener::from_file(format!("{}/process/tokenizer.json", dir), context_length)?,
    })
  }
}

// pub struct ClipImg<Crop: Fn(&RgbImage, (u32, u32), u32) -> RgbImage> {
//   pub env: Arc<Environment>,
//   pub sess: Session,
//   pub dim: u32,
//   pub crop: Crop,
// }
