use std::sync::OnceLock;

use clip_qdrant::qdrant_client;
use clip_runtime::{
  img::{clip_img, ClipImg},
  ort::ClipOrt,
};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use qdrant_client::qdrant::{point_id::PointIdOptions, PointId, SearchPoints};

pub type ClipImgCropTop = ClipImg<clip_img::CropTop>;
pub static ONNX: OnceLock<ClipImgCropTop> = OnceLock::new();
pub static CLIP: &'static str = "clip";

pub fn onnx() -> &'static ClipImgCropTop {
  loop {
    match ONNX.get() {
      Some(r) => return r,
      None => {
        let ort = ClipOrt::new().unwrap();
        let model = ort.model(std::env::var("MODEL_DIR").unwrap());
        let onnx_path = std::env::var("ONNX_IMG").unwrap();
        let onnx = model.img(&onnx_path, 224, clip_img::CropTop()).unwrap();
        let _ = ONNX.set(onnx);
        continue;
      }
    }
  }
}

#[napi]
pub struct Db(String);

#[napi]
impl Db {
  #[napi]
  pub async fn add_if_not_exist(
    &self,
    id: i64,
    payload: String,
    img: Buffer,
    ext: Option<String>,
  ) -> napi::Result<u64> {
    let vector = onnx().encode(ext.as_deref(), &img)?.0;
    let point = SearchPoints {
      collection_name: CLIP.to_string(),
      vector: vector.clone(),
      limit: 1,
      offset: None,
      with_payload: None, //Some(true.into()),
      ..Default::default()
    };
    let client = qdrant_client();
    let response = client.search_points(&point).await?.result;
    if !response.is_empty() {
      let item = response.first().unwrap();
      if let Some(PointId {
        point_id_options: Some(PointIdOptions::Num(id)),
      }) = item.id
      {
        // 结合其他算法去重
        if item.score > 0.99 {
          // cost 大于 0.99 基本可以认为是同一张图
          return Ok(id);
        }
      }
    }
    self._add(id, payload, vector).await?;
    Ok(0)
  }

  async fn _add(&self, id: i64, payload: String, vec: Vec<f32>) -> napi::Result<()> {
    let db = clip_qdrant::Db {
      name: self.0.clone(),
    };
    // let payload = format!("{{\"w\":{w},\"h\":{h},") + &payload[1..];
    db.add(id as u64, vec, &payload).await?;
    Ok(())
  }

  #[napi]
  pub async fn add(
    &self,
    id: i64,
    payload: String,
    img: Buffer,
    ext: Option<String>,
  ) -> napi::Result<()> {
    let vec_w_h = onnx().encode(ext.as_deref(), &img)?;
    self._add(id, payload, vec_w_h.0).await
  }
}

#[napi]
pub fn db_new(name: String) -> Db {
  Db(name)
}
