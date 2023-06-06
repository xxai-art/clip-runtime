mod env;
use std::sync::OnceLock;

use anyhow::Result;
use awp::srv;
use clip_qdrant::qdrant_client;
use clip_runtime::{
  img::{clip_img, ClipImg},
  ort::{ClipModel, ClipOrt},
};

use crate::env::TO;

static ONNX: OnceLock<ClipImg<clip_img::CropTop>> = OnceLock::new();

pub fn clip_onnx() -> &'static ClipImg<clip_img::CropTop> {
  ONNX.get_or_init(|| {
    let ort = ClipOrt::new().unwrap();
    let model = ort.model(std::env::var("MODEL_DIR").unwrap());
    model.img("onnx/ImgNorm", 224, clip_img::CropTop()).unwrap()
  })
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = qdrant_client().await?;
  let onnx = clip_onnx();

  unsafe {
    TO = std::env::var("TO").unwrap();
    tracing::info!("→ {TO}");
  }

  println!("Hello, world!");
  Ok(())
}
