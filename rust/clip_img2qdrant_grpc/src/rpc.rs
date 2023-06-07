use clip_qdrant::Collection;
pub use proto::img_qdrant_server::ImgQdrantServer;
use proto::{AddIn, AddOut};
use tonic::{Request, Response};
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

    let payload = serde_json::from_str::<Payload>(&req.payload)?;
    let vec = crate::img::get(&req.url).await?;

    let point = PointStruct::new(req.id, vec, payload);

    // unsafe {
    //   crate::Q
    //     .get()
    //     .unwrap()
    //     .upsert_points_blocking(&crate::env::CLIP, vec![point], None)
    //     .await?;
    // }
    dbg!("inserted", req);
    Ok(Response::new(AddOut {}))
  }
}
