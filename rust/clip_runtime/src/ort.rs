use std::sync::Arc;

use anyhow::Result;
use clip_img::Croper;
use clip_txt::Tokener;
use ort::{environment::Environment, GraphOptimizationLevel, Session, SessionBuilder};

use crate::{img::ClipImg, providers::providers, txt::ClipTxt};

pub struct ClipOrt {
  pub env: Arc<Environment>,
}

pub struct ClipModel {
  pub dir: String,
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

  pub fn model(&self, dir: impl Into<String>) -> ClipModel {
    ClipModel {
      dir: dir.into(),
      env: self.env.clone(),
    }
  }
}

impl ClipModel {
  pub fn sess(&self, onnx: &str) -> Result<Session> {
    Ok(
      SessionBuilder::new(&self.env)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_inter_threads(num_cpus::get() as i16)?
        .with_model_from_file(format!("{}/{}.onnx", &self.dir, onnx))?,
    )
  }

  pub fn txt(&self, onnx: &str, context_length: usize) -> clip_txt::Result<ClipTxt> {
    Ok(ClipTxt {
      env: self.env.clone(),
      sess: self.sess(onnx)?,
      tokener: Tokener::from_file(
        format!("{}/process/tokenizer.json", self.dir),
        context_length,
      )?,
    })
  }

  pub fn img<C: Croper>(&self, onnx: &str, dim: u32, croper: C) -> clip_txt::Result<ClipImg<C>> {
    Ok(ClipImg {
      env: self.env.clone(),
      sess: self.sess(onnx)?,
      dim,
      croper,
    })
  }
}
