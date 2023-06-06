use anyhow::Result;
use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
  let model_dir = var("MODEL_DIR")?;
  let grpc_port = var("QDRANT__SERVICE__GRPC_PORT")?;
  // let config = QdrantClientConfig::from_url("http://localhost:");

  dbg!(model_dir, grpc_port);
  Ok(())
}
