use std::{env::var};

use anyhow::Result;
pub use qdrant_client::{
  self,
  prelude::{QdrantClient, QdrantClientConfig},
  qdrant::{
    quantization_config::Quantization, vectors_config::Config, CreateCollection, Distance,
    QuantizationConfig, VectorParams, VectorsConfig,
  },
};

pub async fn qdrant_client() -> Result<QdrantClient> {
  let grpc = var("QDRANT_GRPC")?;
  let mut config = QdrantClientConfig::from_url(&grpc);

  if let Ok(key) = var("QDRANT__SERVICE__API_KEY") {
    config.set_api_key(&key);
  }
  let client = QdrantClient::new(Some(config))?;

  Ok(client)
}
