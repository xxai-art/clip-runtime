use std::sync::OnceLock;

use anyhow::Result;
use clip_qdrant::qdrant_client;
use clip_runtime::{
  img::{clip_img, ClipImg},
  ort::{ClipModel, ClipOrt},
};

static MODEL: OnceLock<ClipImg<clip_img::CropTop>> = OnceLock::new();

pub fn clip_model() -> &'static ClipImg<clip_img::CropTop> {
  MODEL.get_or_init(|| {
    let ort = ClipOrt::new().unwrap();
    ort.model(std::env::var("MODEL_DIR").unwrap());
    model.img("onnx/ImgNorm", 224, clip_img::CropTop()).unwrap()
  })
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = qdrant_client().await?;
  let model = clip_model();

  println!("Hello, world!");
  Ok(())
}
