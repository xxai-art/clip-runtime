use anyhow::Result;
use qdrant_client::prelude::QdrantClientConfig;
use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
  let grpc = var("QDRANT_GRPC")?;
  let mut config = QdrantClientConfig::from_url(&grpc);

  if let Ok(key) = var("QDRANT__SERVICE__API_KEY") {
    config.set_api_key(&key);
  }
  let client = QdrantClient::new(Some(config))?;

  let li = client.list_collections().await?;

  dbg!(li);
  // let config = QdrantClientConfig::from_url("http://localhost:");
  // let model_dir = var("MODEL_DIR")?;

  Ok(())
}
