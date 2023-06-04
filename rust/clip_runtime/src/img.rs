use std::sync::Arc;

use anyhow::Result;
use clip_img::{processor, Croper};
use ndarray::{s, Array};
use ort::{
  tensor::{FromArray, InputTensor},
  Environment,
};

use crate::{session::ClipSession, typedef::Arr};

pub struct ClipImg<C: Croper> {
  pub env: Arc<Environment>,
  pub sess: ClipSession,
  pub dim: usize,
  pub croper: C,
}

impl<C: Croper> ClipImg<C> {
  pub fn encode(&self, img: &[u8]) -> Result<Arr> {
    let dim = self.dim;
    let img = processor(img, self.dim as u32, &self.croper)?;
    let mut a = Array::<f32, _>::zeros((
      1, // 有多少图片
      3, dim, dim,
    ));
    a.slice_mut(s![0..1, 0..3, 0..dim, 0..dim]).assign(&img);

    // 批量编码图片的时候可以用这个函数
    // img.iter().enumerate().for_each(|(idx, md_vec)| {
    //   a.slice_mut(s![idx..idx + 1, 0..3, 0..dim, 0..dim])
    //     .assign(&md_vec);
    // });

    self.sess.run([InputTensor::from_array(a.into_dyn())])
  }
}

#[test]
fn test_image_encode() -> Result<()> {
  use crate::ort::ClipOrt;
  let dir = std::env::current_dir()?;

  let ort = ClipOrt::new()?;
  let model = ort.model(dir.join("model/AltCLIP-XLMR-L-m18").to_str().unwrap());
  let clip_img = model.img("onnx/Img", 224, clip_img::CropCenter())?;

  let img = std::fs::read(dir.join("cat.jpg"))?;
  let out = clip_img.encode(&img)?;
  println!("img {}", out);
  Ok(())
}
