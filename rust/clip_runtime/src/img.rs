use std::sync::Arc;

use anyhow::Result;
pub use clip_img::{self, processor, Croper};
use ndarray::{s, Array};
use ort::Environment;

use crate::{run, session::ClipSession};

pub struct ClipImg<C: Croper> {
  pub env: Arc<Environment>,
  pub sess: ClipSession,
  pub dim: usize,
  pub croper: C,
}

impl<C: Croper> ClipImg<C> {
  pub fn encode(&self, ext: Option<&str>, img: &[u8]) -> Result<(Vec<f32>, u32, u32)> {
    let dim = self.dim;
    let (img, w, h) = processor(ext, img, self.dim as u32, &self.croper)?;
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

    Ok((run!(self.sess, a)?.into_raw_vec(), w, h))
  }
}

#[test]
fn test_image_encode() -> Result<()> {
  let model = crate::test::clip_model();
  let dir = crate::test::root();
  let clip_img = model.img("onnx/ImgNorm", 224, clip_img::CropCenter())?;

  let img = std::fs::read(dir.join("img/cat.jpg"))?;
  let (out, w, h) = clip_img.encode(Some("jpg"), &img)?;
  // println!("img w {w} h {h} {out}");
  Ok(())
}
