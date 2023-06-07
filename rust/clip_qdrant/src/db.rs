use anyhow::Result;
use qdrant_client::{
  prelude::Payload,
  qdrant::{
    quantization_config::Quantization, vectors_config::Config, CreateCollection, Distance,
    PointStruct, QuantizationConfig, VectorParams, VectorsConfig,
  },
};

use crate::qdrant_client;

#[derive(Debug)]
pub struct Db {
  pub name: String,
  pub size: u64,
}

#[macro_export]
macro_rules! db {
    ($name:ident, $size:expr) => {
        $crate::paste! {
        pub static [<DB_ $name:snake:upper>]: std::sync::OnceLock<$crate::Db> = std::sync::OnceLock::new();

        pub async fn [<db_ $name>]() -> anyhow::Result<&'static clip_qdrant::Db> {
            loop {
                match [<DB_ $name:snake:upper>].get() {
                    Some(r) => return Ok(r),
                    None => {
                        let _  = [<DB_ $name:snake:upper>].set(
                            $crate::Db {
                                name:stringify!($name).to_string(),
                                size:$size
                            }
                        );
                        continue;
                    }
                }
            }
        }
    }
  }
}

impl Db {
  pub async fn add(&self, id: u64, vec: Vec<f32>, payload: &str) -> Result<()> {
    let client = qdrant_client().await?;
    let payload = serde_json::from_str::<Payload>(payload)?;
    let point = PointStruct::new(id, vec, payload);

    client
      .upsert_points_blocking(&self.name, vec![point], None)
      .await?;

    Ok(())
  }

  pub async fn init(&self) -> Result<()> {
    let client = qdrant_client().await?;
    client
      .create_collection(&CreateCollection {
        collection_name: self.name.clone(),
        vectors_config: Some(VectorsConfig {
          config: Some(Config::Params(VectorParams {
            size: self.size,
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
    Ok(())
  }
}

// use std::{collections::BTreeSet, env::var, sync::OnceLock};
//
// use clip_qdrant::{
//   qdrant_client, Config, CreateDb, Distance, QdrantClient, Quantization,
//   QuantizationConfig, VectorParams, VectorsConfig,
// };
//
// pub async fn create_collection(name: &str) -> anyhow::Result<()> {
//   let client = qdrant_client().await?;
//   let li = client
//     .list_collections()
//     .await?
//     .collections
//     .into_iter()
//     .map(|i| i.name)
//     .collect::<BTreeSet<_>>();
//
//   // let name = var("CLIP_NAME").unwrap_or_else(|_| "clip".into());
//
//   if li.get(&name).is_none() {
//   }
//   let _ = Q.set(client);
//   Ok(())
// }
