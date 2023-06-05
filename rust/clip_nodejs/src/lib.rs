use anyhow::Result;
use clip_runtime::ort::{ClipModel, ClipOrt};
use napi::JsNumber;
use napi_derive::napi;

#[napi]
pub struct Model(ClipModel);

#[napi]
impl Model {
  #[napi(constructor)]
  pub fn new(dir: String) -> Result<Self> {
    Ok(Self(ClipOrt::new()?.model(dir)))
  }
}

#[napi]
pub fn hello_world(li: Vec<JsNumber>) -> Result<i64> {
  Ok(li.len() as i64)
}
