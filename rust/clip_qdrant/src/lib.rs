pub mod db;
use std::env::var;

pub use async_lazy::Lazy;
use ctor::ctor;
pub use db::Db;
pub use paste::paste;
use qdrant_client::prelude::{QdrantClient, QdrantClientConfig};

static Q: Lazy<QdrantClient> = Lazy::const_new(|| {
  Box::pin(async move {
    let uri = "QDRANT_GRPC";
    let grpc = var(uri).expect(&format!("MISS ENV {uri}"));
    tracing::info!("connecting to qdrant {grpc}");
    let mut config = QdrantClientConfig::from_url(&grpc);
    if let Ok(key) = var("QDRANT__SERVICE__API_KEY") {
      config.set_api_key(&key);
    }
    QdrantClient::new(Some(config)).expect(&format!("CAN'T CONNECT {uri}"))
  })
});

#[ctor]
fn init() {
  trt::TRT.block_on(async move {
    use std::future::IntoFuture;
    Q.into_future().await;
  })
}

pub fn qdrant_client() -> &'static QdrantClient {
  Q.get().unwrap()
}
