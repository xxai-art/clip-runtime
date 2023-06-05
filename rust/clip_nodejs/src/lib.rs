use anyhow::Result;
use clip_runtime::{
  img::ClipImg,
  ort::{ClipModel, ClipOrt},
  txt::ClipTxt,
};
use napi::Either;
use napi_derive::napi;

#[napi]
pub struct Arr(clip_runtime::typedef::Arr);

#[napi]
pub struct Model(ClipModel);

#[napi]
pub struct Txt(ClipTxt);

// #[napi]
// pub struct Img(ClipImg);

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
  pub fn len(&self) -> usize {
    self.0.len()
  }

  #[napi]
  pub fn shape(&self) -> &[usize] {
    self.0.shape()
  }
}
