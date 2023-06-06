pub use proto::img_qdrant_server::ImgQdrantServer;
use proto::{AddIn, AddOut};
use serde_json::json;
use tonic::{transport::Server, Request, Response, Status};
use tonic_catch::{tonic_catch, Error, Result};

pub mod proto {
  tonic::include_proto!("img2qdrant");
}

#[derive(Debug, Default)]
pub struct ImgQdrant {}

#[tonic_catch]
impl proto::img_qdrant_server::ImgQdrant for ImgQdrant {
  async fn add(&self, req: Request<AddIn>) -> Result<AddOut> {
    let req = req.get_ref();
    let vec = crate::img::get(&req.url).await?;
    let q = crate::Q.get().unwrap();
    Ok(Response::new(AddOut {}))
  }
}
