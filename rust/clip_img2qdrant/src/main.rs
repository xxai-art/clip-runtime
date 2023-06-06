mod env;
mod img;

use std::sync::OnceLock;

use anyhow::Result;
use awp::srv;
use axum::{routing::get, Router};
use clip_qdrant::qdrant_client::{self, QdrantClient};
use clip_runtime::{
  img::{clip_img, ClipImg},
  ort::{ClipModel, ClipOrt},
};

use crate::env::TO;

pub static ONNX: OnceLock<ClipImg<clip_img::CropTop>> = OnceLock::new();
pub static ONNX: OnceLock = OnceLock::new();

pub fn clip_onnx() -> &'static ClipImg<clip_img::CropTop> {
  ONNX.get_or_init(|| {
    let ort = ClipOrt::new().unwrap();
    let model = ort.model(std::env::var("MODEL_DIR").unwrap());
    model.img("onnx/ImgNorm", 224, clip_img::CropTop()).unwrap()
  })
}

#[tokio::main]
async fn main() -> Result<()> {
  awp::init();
  let client = qdrant_client().await?;
  let onnx = clip_onnx();

  unsafe {
    TO = std::env::var("TO").unwrap_or("".to_string());
    tracing::info!("→ {TO}");
  }

  let mut router = Router::new();
  router = router.route(r"/*url", get(crate::img::get));
  awp::srv(router, 5550).await;
  Ok(())
}
