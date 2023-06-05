use anyhow::Result;
use clip_runtime::{
  img::ClipImg,
  ort::{ClipModel, ClipOrt},
  txt::ClipTxt,
};
use napi::JsNumber;
use napi_derive::napi;

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
impl Txt {}
