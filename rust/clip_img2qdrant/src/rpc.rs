pub use img2qdrant::img_qdrant_server::ImgQdrantServer;
use img2qdrant::{AddIn, AddOut};
use tonic::{transport::Server, Request, Response, Status};

pub mod img2qdrant {
  tonic::include_proto!("img2qdrant");
}

#[derive(Debug, Default)]
pub struct ImgQdrant {}

#[tonic::async_trait]
impl img2qdrant::img_qdrant_server::ImgQdrant for ImgQdrant {
  async fn add(
    &self,
    req: Request<AddIn>, // 接收以HelloRequest为类型的请求
  ) -> Result<Response<AddOut>, Status> {
    let req = req.get_ref();
    println!("{:?}", req);
    let vec = crate::img::get(&req.url).await?;

    let reply = AddOut {};

    Ok(Response::new(reply)) // 发回格式化的问候语
  }
}
