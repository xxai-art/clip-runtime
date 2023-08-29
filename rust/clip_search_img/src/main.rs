use ctor::ctor;

mod env;
mod img;
mod log;
mod rpc;
use std::sync::OnceLock;

use anyhow::Result;
use clip_runtime::{
  img::{clip_img, ClipImg},
  ort::ClipOrt,
};
use tonic::transport::Server;

use crate::{
  env::TO,
  rpc::{QServer, Q},
};
pub static ONNX: OnceLock<ClipImg<clip_img::CropTop>> = OnceLock::new();

#[ctor]
fn init() {
  log::init();
}

fn main() -> Result<()> {
  let model_name = "onnx/Img";
  tracing::info!("LOADING MODEL {model_name}");
  let _ = ONNX.set({
    let ort = ClipOrt::new().unwrap();
    let model = ort.model(std::env::var("MODEL_DIR").unwrap());
    model.img(model_name, 224, clip_img::CropTop()).unwrap()
  });

  tracing::info!("MODEL LOADED");

  let port = std::env::var("PORT").unwrap_or("8662".to_string());
  let addr = format!("0.0.0.0:{}", port).parse()?;
  tracing::info!("grpc://{}", addr);

  unsafe {
    TO = std::env::var("TO").unwrap_or("".to_string());
    tracing::info!("→ {TO}");
  }

  trt::TRT.block_on(async move {
    let img_qdrant = Q::default();
    Server::builder()
      .add_service(QServer::new(img_qdrant))
      .serve(addr)
      .await?;
    Ok::<(), anyhow::Error>(())
  })?;

  Ok(())
}
