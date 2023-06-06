use anyhow::Result;
use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
  let model_dir = var("MODEL_DIR")?;
  let grpc = var("QDRANT_GRPC")?;
  // let config = QdrantClientConfig::from_url("http://localhost:");

  dbg!(model_dir, grpc);
  Ok(())
}
