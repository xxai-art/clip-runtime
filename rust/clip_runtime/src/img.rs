use std::sync::Arc;

use anyhow::Result;
use clip_img::{processor, Croper};
use ndarray::{s, Array};
use ort::{
  tensor::{FromArray, InputTensor},
  Environment,
};

use crate::session::ClipSession;

pub struct ClipImg<C: Croper> {
  pub env: Arc<Environment>,
  pub sess: ClipSession,
  pub dim: usize,
  pub croper: C,
}

impl<C: Croper> ClipImg<C> {
  pub fn encode(&self, img: &[u8]) -> Result<()> {
    let dim = self.dim;
    let img = processor(img, self.dim as u32, &self.croper)?;
    let mut a = Array::<f32, _>::zeros((
      1, // 有多少图片
      3, dim, dim,
    ));
    a.slice_mut(s![0..1, 0..3, 0..dim, 0..dim]).assign(&img);

    // img.iter().enumerate().for_each(|(idx, md_vec)| {
    //   a.slice_mut(s![idx..idx + 1, 0..3, 0..dim, 0..dim])
    //     .assign(&md_vec);
    // });

    let _out = &self.sess.run([InputTensor::from_array(a.into_dyn())])?;

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
  let img = std::fs::read(fp)?;
  clip_img.encode(&img)?;
  Ok(())
}
