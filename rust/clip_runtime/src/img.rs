use std::sync::Arc;

use anyhow::Result;
use clip_img::Croper;
use ort::{Environment, Session};

pub struct ClipImg<C: Croper> {
  pub env: Arc<Environment>,
  pub sess: Session,
  pub dim: u32,
  pub croper: C,
}

impl<C: Croper> ClipImg<C> {
  pub fn encode(img: &[u8]) -> Result<()> {
    Ok(())
  }
}

#[test]
fn test_image_encode() -> Result<()> {
  // let mut fp: std::path::PathBuf = std::env::current_dir()?;
  // fp.push("cat.jpg");
  // let fp = fp.display().to_string();
  // let img = std::fs::read(&fp)?;
  // let dim = 224;
  // let img = crate::processor(&img, dim, crate::CropCenter())?;
  // to_png(img, &(fp.clone() + ".png"))?;
  // let py_img = json_to_narray(&(fp.clone() + ".json"))?;
  // to_png(py_img, &(fp + ".py.png"))?;
  Ok(())
}
