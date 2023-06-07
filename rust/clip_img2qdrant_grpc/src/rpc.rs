use clip_qdrant::db;
pub use proto::img_qdrant_server::ImgQdrantServer;
use proto::{AddIn, AddOut};
use tonic::{Request, Response};
use tonic_catch::{tonic_catch, Error, Result};

pub mod proto {
  tonic::include_proto!("img2qdrant");
}

db!(clip, 1024);

#[derive(Debug, Default)]
pub struct ImgQdrant {}

#[tonic_catch]
impl proto::img_qdrant_server::ImgQdrant for ImgQdrant {
  async fn add(&self, req: Request<AddIn>) -> Result<AddOut> {
    let req = req.get_ref();
    let vec = crate::img::get(&req.url).await?;

    db_clip().await?.add(req.id, vec, &req.payload).await?;
    Ok(Response::new(AddOut {}))
  }
}
