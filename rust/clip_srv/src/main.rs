use std::{collections::BTreeSet, env::var};

use anyhow::Result;
use qdrant_client::{
  prelude::{QdrantClient, QdrantClientConfig},
  qdrant::{
    quantization_config::Quantization, vectors_config::Config, CreateCollection, Distance,
    QuantizationConfig, VectorParams, VectorsConfig,
  },
};

async fn qdrant_client() -> Result<QdrantClient> {
  let grpc = var("QDRANT_GRPC")?;
  let mut config = QdrantClientConfig::from_url(&grpc);

  if let Ok(key) = var("QDRANT__SERVICE__API_KEY") {
    config.set_api_key(&key);
  }
  let client = QdrantClient::new(Some(config))?;

  let li = client
    .list_collections()
    .await?
    .collections
    .into_iter()
    .map(|i| i.name)
    .collect::<BTreeSet<_>>();

  let name = var("CLIP_NAME").unwrap_or_else(|_| "clip".into());
  if li.get(&name).is_none() {
    client
      .create_collection(&CreateCollection {
        collection_name: name,
        vectors_config: Some(VectorsConfig {
          config: Some(Config::Params(VectorParams {
            size: 1024,
            distance: Distance::Euclid.into(),
            on_disk: Some(true),
            hnsw_config: None,
            quantization_config: Some(QuantizationConfig {
              quantization: Quantization::Scalar(qdrant_client::qdrant::ScalarQuantization {
                r#type: qdrant_client::qdrant::QuantizationType::Int8 as i32,
                quantile: Some(0.99),
                always_ram: Some(false),
              })
              .into(),
            }),
          })),
        }),
        ..Default::default()
      })
      .await?;
  }

  Ok(client)
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = qdrant_client().await?;
  // let model_dir = var("MODEL_DIR")?;
  Ok(())
}
