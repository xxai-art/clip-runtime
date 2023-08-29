#![feature(macro_metavar_expr)]

pub mod img;
pub mod ort;
pub mod providers;
pub mod session;
pub mod txt;
pub mod typedef;
pub mod util;
use ndarray::Axis;

use crate::typedef::Arr;

pub fn softmax(input: &Arr) -> Arr {
  let mut output = Arr::zeros(input.raw_dim());
  for (in_row, mut out_row) in input.axis_iter(Axis(0)).zip(output.axis_iter_mut(Axis(0))) {
    let max = in_row.iter().fold(f32::MIN, |max, &val| max.max(val));
    let exps = in_row.map(|&x| (x - max).exp());
    let exp_sum = exps.sum();
    out_row.assign(&(exps / exp_sum));
  }
  output
}
// pub fn softmax(vec: &[f32]) -> Vec<f32> {
//   let max = vec.iter().fold(f32::MIN, |a, &b| a.max(b));
//   let mut exps: Vec<f32> = vec.iter().map(|x| (x - max).exp()).collect();
//   let sum: f32 = exps.iter().sum();
//   exps.iter_mut().for_each(|x| *x /= sum);
//   exps
// }

pub fn cls(txt_feature: &Arr, img_feature: &Arr) -> Vec<f32> {
  let li = img_feature.dot(&txt_feature.t());
  softmax(&li).into_raw_vec()
}

#[cfg(test)]
mod test {
  use std::sync::OnceLock;

  use crate::{
    cls,
    ort::{ClipModel, ClipOrt},
  };

  static MODEL: OnceLock<ClipModel> = OnceLock::new();

  pub fn root() -> std::path::PathBuf {
    std::env::current_dir()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("lib")
  }

  pub fn clip_model() -> &'static ClipModel {
    MODEL.get_or_init(|| {
      let ort = ClipOrt::new().unwrap();
      ort.model(std::env::var("MODEL_DIR").unwrap())
    })
  }

  #[test]
  fn init() {
    tracing_subscriber::fmt::init();
  }

  macro_rules! tmpl_kind_li {
    ($tmpl:expr, $($x:expr),* $(,)? ) => {{
      [ $(format!($tmpl, $x)),* ]
    }};
  }

  #[test]
  fn test() -> anyhow::Result<()> {
    let model = clip_model();
    let clip_txt = model.txt("onnx/Txt", 77)?;

    let prompt_li = [
      tmpl_kind_li!("a photo of {}", "cat", "rat", "dog", "man", "woman"),
      tmpl_kind_li!("一张{}的图片", "猫", "老鼠", "狗", "男人", "女人"),
    ];

    let txt_feature_li = prompt_li
      .iter()
      .map(|word_li| clip_txt.encode_batch(word_li.clone().into_iter()).unwrap())
      .collect::<Vec<_>>();

    let clip_img = model.img("onnx/Img", 224, clip_img::CropCenter())?;

    let dir = crate::test::root();
    for file in ["cat", "build"] {
      let fp = dir.join(format!("img/{}.jpg", file));

      let fp = fp.display().to_string();
      let (img_feature, ..) = clip_img.encode(Some("jpg"), &std::fs::read(fp)?)?;

      // println!("\n❯ {}", file);
      // for (txt_feature, word_li) in txt_feature_li.iter().zip(prompt_li.iter()) {
      //   let text_probs = cls(txt_feature, ndarray::arr1(&img_feature));
      //   for (p, word) in text_probs.iter().zip(word_li.iter()) {
      //     println!("{} {:.2}%", word, (*p) * 100.0)
      //   }
      // }
    }
    Ok(())
  }
}
