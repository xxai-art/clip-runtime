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

// pub struct TaskAdd {
//   id: i64,
//   payload: String,
//   img: Buffer,
//   ext: Option<String>,
//   db: String,
// }
//
// impl Task for TaskAdd {
//   type Output = ();
//   type JsValue = ();
//
//   fn compute(&mut self) -> napi::Result<Self::Output> {
//     Ok(())
//   }
//
//   fn resolve(&mut self, _: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
//     Ok(())
//   }
// }

#[napi]
pub struct Db(String);

#[napi]
impl Db {
  #[napi]
  pub async fn add(
    &self,
    id: i64,
    payload: String,
    img: Buffer,
    ext: Option<String>,
  ) -> napi::Result<()> {
    let vec = onnx().encode(ext.as_deref(), &img)?.into_raw_vec();
    let db = clip_qdrant::Db {
      name: self.0.clone(),
    }
    .add(id as u64, vec, &payload)
    .await?;
    Ok(())
  }
}

#[napi]
pub fn db_new(name: String) -> Db {
  Db(name)
}
