use std::sync::OnceLock;

use anyhow::Result;
use clip_qdrant::qdrant_client;
use clip_runtime::{
  img::clip_img,
  ort::{ClipModel, ClipOrt},
};

static MODEL: OnceLock<ClipModel> = OnceLock::new();

pub fn clip_model() -> &'static ClipModel {
  MODEL.get_or_init(|| {
    let ort = ClipOrt::new().unwrap();
    ort.model(std::env::var("MODEL_DIR").unwrap())
  })
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = qdrant_client().await?;
  let model = clip_model();
  let clip_img = model.img("onnx/ImgNorm", 224, clip_img::CropCenter())?;

  println!("Hello, world!");
  Ok(())
}
