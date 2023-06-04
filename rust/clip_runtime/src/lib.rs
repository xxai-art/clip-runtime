mod util;

mod txt;
pub use txt::ClipTxt;

pub mod providers;
use std::{path::Path, sync::Arc};

use anyhow::Result;
use clip_txt::Tokener;
use ort::{environment::Environment, GraphOptimizationLevel, SessionBuilder};

use crate::providers::providers;

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

#[cfg(test)]
mod tests {
  use clip_txt::Result;

  use super::*;

  #[test]
  fn test() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut dir = std::env::current_dir()?;
    dir.push("model/AltCLIP-XLMR-L-m18");

    let ort = ClipOrt::new()?;
    let clip_txt = ort.txt(dir, "onnx/Txt", 77)?;
    let word_li = [
      "a photo for chinese woman",
      "a photo of dog",
      "a photo for cat",
    ];
    for word in word_li {
      let out = clip_txt.encode(word)?;
      println!("❯ {}\n{}\n", word, out);
    }
    let out = clip_txt.encode_batch(word_li)?;
    for (out, word) in out.rows().into_iter().zip(word_li.iter()) {
      println!("❯ {}\n{}\n", word, out);
    }
    Ok(())
  }
}
