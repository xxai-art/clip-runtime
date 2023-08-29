use clip_qdrant::db;
pub use proto::q_server::QServer;
use proto::{QIn, QOut};
use tonic::{Request, Response};
use tonic_catch::{tonic_catch, Error, Result};

pub mod proto {
  tonic::include_proto!("rpc");
}

db!(clip);

#[derive(Debug, Default)]
pub struct Q {}

#[tonic_catch]
impl proto::q_server::Q for Q {
  async fn add(&self, req: Request<QIn>) -> Result<QOut> {
    let req = req.get_ref();
    let vec = crate::img::get(&req.url).await?;
    db_clip().add(req.id, vec, &req.payload).await?;
    Ok(Response::new(QOut {}))
  }
}
