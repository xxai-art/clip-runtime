use std::sync::{Arc, OnceLock};

use anyhow::Result;
use clip_img::Croper;
use clip_txt::Tokener;
use ort::{environment::Environment, OrtError};

static ORT: OnceLock<Result<Arc<Environment>, OrtError>> = OnceLock::new();

use crate::{img::ClipImg, providers::providers, session::ClipSession, txt::ClipTxt};

pub struct ClipOrt {
  pub env: Arc<Environment>,
}

pub struct ClipModel {
  pub dir: String,
  pub env: Arc<Environment>,
}

impl ClipOrt {
  pub fn new() -> Result<Self> {
    match ORT.get_or_init(|| {
      let provider = providers();
      let mut use_provider: String = "CPU".into();
      for i in &provider {
        if i.is_available() {
          use_provider = format!("{:?}", i);
          break;
        }
      }
      println!("\nONNX PROVIDER {use_provider}\n");

      Ok(
        Environment::builder()
          .with_name("clip")
          .with_execution_providers(provider)
          .build()?
          .into_arc(),
      )
    }) {
      Ok(env) => Ok(ClipOrt { env: env.clone() }),
      Err(err) => Err(err.into()),
    }
  }

  pub fn model(&self, dir: impl Into<String>) -> ClipModel {
    ClipModel {
      dir: dir.into(),
      env: self.env.clone(),
    }
  }
}

impl ClipModel {
  pub fn sess(&self, onnx: &str) -> Result<ClipSession> {
    ClipSession::new(&self.dir, &self.env, onnx)
  }

  pub fn txt(&self, onnx: &str, context_length: usize) -> Result<ClipTxt> {
    Ok(ClipTxt {
      env: self.env.clone(),
      sess: self.sess(onnx)?,
      tokener: Tokener::from_file(
        format!("{}/process/tokenizer.json", self.dir),
        context_length,
      )?,
    })
  }

  pub fn img<C: Croper>(&self, onnx: &str, dim: usize, croper: C) -> anyhow::Result<ClipImg<C>> {
    Ok(ClipImg {
      env: self.env.clone(),
      sess: self.sess(onnx)?,
      dim,
      croper,
    })
  }
}
