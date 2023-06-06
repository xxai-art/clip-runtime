#![feature(impl_trait_in_assoc_type)]
mod env;
mod log;
mod srv;

use std::{env::var, sync::OnceLock};

use anyhow::Result;
use axum::{routing::get, Router};
use clip_qdrant::qdrant_client;
use clip_runtime::ort::{ClipModel, ClipOrt};

static MODEL: OnceLock<ClipModel> = OnceLock::new();

pub fn clip_model() -> &'static ClipModel {
  MODEL.get_or_init(|| {
    let ort = ClipOrt::new().unwrap();
    ort.model(std::env::var("MODEL_DIR").unwrap())
  })
}

#[tokio::main]
async fn main() -> Result<()> {
  crate::log::init();

  let client = qdrant_client().await?;
  let model = clip_model();
  let mut router = Router::new();

  // router = router.route(r"/*li", get(crate::url::img::get));
  if var("TXT").is_ok() {
    let clip_txt = model.txt("onnx/Txt", 77)?;
  }
  srv::srv(router, 3678).await;
  Ok(())
}
