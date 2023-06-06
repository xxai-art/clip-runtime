mod env;
mod img;
mod rpc;
use std::sync::OnceLock;

use anyhow::Result;
use clip_qdrant::{qdrant_client, QdrantClient};
use clip_runtime::{
  img::{clip_img, ClipImg},
  ort::{ClipModel, ClipOrt},
};
use tonic::{transport::Server, Request, Response, Status};

use crate::{
  env::TO,
  rpc::{ImgQdrant, ImgQdrantServer},
};

pub static ONNX: OnceLock<ClipImg<clip_img::CropTop>> = OnceLock::new();
pub static Q: OnceLock<QdrantClient> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
  let client = qdrant_client().await?;
  let _ = Q.set(client);
  let _ = ONNX.set({
    let ort = ClipOrt::new().unwrap();
    let model = ort.model(std::env::var("MODEL_DIR").unwrap());
    model.img("onnx/ImgNorm", 224, clip_img::CropTop()).unwrap()
  });

  unsafe {
    TO = std::env::var("TO").unwrap_or("".to_string());
    tracing::info!("→ {TO}");
  }

  let port = std::env::var("PORT").unwrap_or("8662".to_string());

  let addr = format!("0.0.0.0:{}", port).parse()?;

  println!("grpc://{}", addr);

  let img_qdrant = ImgQdrant::default();
  Server::builder()
    .add_service(ImgQdrantServer::new(img_qdrant))
    .serve(addr)
    .await?;

  Ok(())
}
