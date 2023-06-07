use anyhow::{anyhow, Result};
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
}

#[macro_export]
macro_rules! db {
    ($name:ident) => {
        $crate::paste! {
            pub static [<DB_ $name:snake:upper>]: std::sync::OnceLock<$crate::Db> = std::sync::OnceLock::new();

            pub fn [<db_ $name>]() -> &'static clip_qdrant::Db {
                loop {
                    match [<DB_ $name:snake:upper>].get() {
                        Some(r) => return r,
                        None => {
                            let _  = [<DB_ $name:snake:upper>].set(
                                $crate::Db {
                                    name:stringify!($name).to_string(),
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
}
