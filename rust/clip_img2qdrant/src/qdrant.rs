use std::{collections::BTreeSet, env::var};

use clip_qdrant::{
  qdrant_client, Config, CreateCollection, Distance, QdrantClient, Quantization,
  QuantizationConfig, VectorParams, VectorsConfig,
};
pub static Q: OnceLock<QdrantClient> = OnceLock::new();

pub async fn init() -> anyhow::Result<()> {
  let client = qdrant_client().await?;
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
  let client = qdrant::init().await?;
  Q.set(client);
  Ok(())
}
