use anyhow::Result;
use clip_runtime::{
  img::{clip_img::CropCenter, ClipImg},
  ort::{ClipModel, ClipOrt},
  txt::ClipTxt,
};
use napi::{bindgen_prelude::Buffer, Either};
use napi_derive::napi;

#[napi]
pub struct Arr(clip_runtime::typedef::Arr);

#[napi]
pub struct Model(ClipModel);

#[napi]
pub struct Txt(ClipTxt);

#[napi]
pub struct Img(ClipImg<CropCenter>);

#[napi]
pub fn cls_img(txt_feature: &Arr, img_feature: &Arr) -> Vec<f32> {
  clip_runtime::cls(&txt_feature.0, &img_feature.0)
}

#[napi]
impl Model {
  #[napi(constructor)]
  pub fn new(dir: String) -> Result<Self> {
    Ok(Self(ClipOrt::new()?.model(dir)))
  }

  #[napi]
  pub fn txt(&self, onnx: String, context_length: u32) -> anyhow::Result<Txt> {
    Ok(Txt(self.0.txt(&onnx, context_length as usize)?))
  }

  #[napi]
  pub fn img(&self, onnx: String, dim: u32) -> anyhow::Result<Img> {
    Ok(Img(self.0.img(&onnx, dim as usize, CropCenter())?))
  }
}

#[napi]
impl Img {
  #[napi]
  pub fn encode(&self, ext: Option<&str>, bin: Buffer) -> Result<Arr> {
    Ok(Arr(self.0.encode(ext, bin.as_ref())?))
  }
}

#[napi]
impl Txt {
  #[napi]
  pub fn encode(&self, txt: Either<Vec<String>, String>) -> Result<Arr> {
    Ok(Arr(match txt {
      Either::A(li) => self.0.encode_batch(li.into_iter()),
      Either::B(txt) => self.0.encode(txt),
    }?))
  }
}

#[napi]
impl Arr {
  #[napi]
  pub fn raw(&self) -> &[f32] {
    self.0.as_slice().unwrap()
  }

  #[napi]
  pub fn width(&self) -> usize {
    self.shape()[1]
  }

  #[napi]
  pub fn shape(&self) -> &[usize] {
    self.0.shape()
  }
}
