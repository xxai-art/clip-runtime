use clip_qdrant::db;
pub use proto::img_qdrant_server::ImgQdrantServer;
use proto::{AddIn, AddOut};
use tonic::{Request, Response};
use tonic_catch::{tonic_catch, Error, Result};

pub mod proto {
  tonic::include_proto!("img2qdrant");
}

db!(clip);

#[derive(Debug, Default)]
pub struct ImgQdrant {}

#[tonic_catch]
impl proto::img_qdrant_server::ImgQdrant for ImgQdrant {
  async fn add(&self, req: Request<AddIn>) -> Result<AddOut> {
    let req = req.get_ref();
    let vec = crate::img::get(&req.url).await?;

    // let _ = db_clip().init().await?;
    db_clip().add(req.id, vec, &req.payload).await?;
    dbg!(req.id);
    Ok(Response::new(AddOut {}))
  }
}
