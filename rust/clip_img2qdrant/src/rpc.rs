pub use proto::img_qdrant_server::ImgQdrantServer;
use proto::{AddIn, AddOut};
use tonic::{transport::Server, Request, Response, Status};
use tonic_catch::{tonic_catch, Error, Result};

pub mod proto {
  tonic::include_proto!("img2qdrant");
}

#[derive(Debug, Default)]
pub struct ImgQdrant {}

#[tonic_catch]
impl proto::img_qdrant_server::ImgQdrant for ImgQdrant {
  async fn add(
    &self,
    req: Request<AddIn>, // 接收以HelloRequest为类型的请求
  ) -> Result<AddOut> {
    let req = req.get_ref();
    println!("{:?}", req);
    let vec = crate::img::get(&req.url).await?;

    let reply = AddOut {};

    Ok(Response::new(reply))
  }
}
