use std::sync::OnceLock;

use anyhow::Result;
use clip_runtime::{
  img::{clip_img, ClipImg},
  ort::ClipOrt,
};
use napi::{
  bindgen_prelude::{AsyncTask, Buffer},
  Env, JsNumber, Task,
};
use napi_derive::napi;

pub type ClipImgCropTop = ClipImg<clip_img::CropTop>;
pub static ONNX: OnceLock<ClipImgCropTop> = OnceLock::new();

pub fn onnx() -> &'static ClipImgCropTop {
  loop {
    match ONNX.get() {
      Some(r) => return r,
      None => {
        let ort = ClipOrt::new().unwrap();
        let model = ort.model(std::env::var("MODEL_DIR").unwrap());
        let onnx = model.img("onnx/ImgNorm", 224, clip_img::CropTop()).unwrap();
        let _ = ONNX.set(onnx);
        continue;
      }
    }
  }
}

#[napi]
struct Db(clip_qdrant::Db);

#[napi]
impl Db {
  #[napi(constructor)]
  pub fn new(&self, name: String) -> Self {
    Self(clip_qdrant::Db { name })
  }
}

pub struct AsyncArg(i64, Buffer);

impl Task for AsyncArg {
  type Output = i64;
  type JsValue = i64;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0 + self.1.len() as i64)
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn async_add(a: i64, b: Buffer) -> AsyncTask<AsyncArg> {
  AsyncTask::new(AsyncArg(a, b))
}
