use anyhow::Result;
use clip_runtime::{
  img::ClipImg,
  ort::{ClipModel, ClipOrt},
  txt::ClipTxt,
};
use napi::JsNumber;
use napi_derive::napi;

#[napi]
pub struct _Model(ClipModel);

#[napi]
pub struct _Txt(ClipTxt);

// #[napi]
// pub struct _Img(ClipImg);

#[napi]
impl _Model {
  #[napi(constructor)]
  pub fn new(dir: String) -> Result<Self> {
    Ok(Self(ClipOrt::new()?.model(dir)))
  }

  #[napi]
  pub fn txt(&self, onnx: String, context_length: u32) -> anyhow::Result<_Txt> {
    Ok(_Txt(self.0.txt(&onnx, context_length as usize)?))
  }
}

#[allow(non_snake_case)]
#[napi]
pub fn Model(dir: String) -> Result<_Model> {
  _Model::new(dir)
}
