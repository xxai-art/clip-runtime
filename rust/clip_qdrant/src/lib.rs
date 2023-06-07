pub mod collection;
use std::{env::var, sync::OnceLock};

use anyhow::Result;
use qdrant_client::{
  prelude::{QdrantClient, QdrantClientConfig},
};

pub static Q: OnceLock<QdrantClient> = OnceLock::new();

pub async fn qdrant_client() -> Result<&'static QdrantClient> {
  loop {
    match Q.get() {
      Some(r) => return Ok(r),
      None => {
        let grpc = var("QDRANT_GRPC")?;
        let mut config = QdrantClientConfig::from_url(&grpc);

        if let Ok(key) = var("QDRANT__SERVICE__API_KEY") {
          config.set_api_key(&key);
        }
        let client = QdrantClient::new(Some(config))?;
        if let Err(client) = Q.set(client) {
          drop(client);
        }
        continue;
      }
    }
  }
}
