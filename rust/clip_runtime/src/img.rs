use std::sync::Arc;

use anyhow::Result;
use clip_img::{processor, Croper};
use ort::{Environment, Session};

pub struct ClipImg<C: Croper> {
  pub env: Arc<Environment>,
  pub sess: Session,
  pub dim: u32,
  pub croper: C,
}

impl<C: Croper> ClipImg<C> {
  pub fn encode(&self, img: &[u8]) -> Result<()> {
    let img = processor(&img, self.dim, &self.croper)?;
    dbg!(img);
    Ok(())
  }
}

#[test]
fn test_image_encode() -> Result<()> {
  use crate::ort::ClipOrt;
  let mut dir = std::env::current_dir()?;
  dir.push("model/AltCLIP-XLMR-L-m18");

  let ort = ClipOrt::new()?;
  let model = ort.model(dir.display().to_string());
  let clip_img = model.img("onnx/Img", 224, clip_img::CropCenter())?;

  let mut fp: std::path::PathBuf = std::env::current_dir()?;
  fp.push("cat.jpg");
  let fp = fp.display().to_string();
  let img = std::fs::read(&fp)?;
  clip_img.encode(&img)?;
  Ok(())
}
