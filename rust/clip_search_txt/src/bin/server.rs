#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;

use clip_search_txt::S;
use volo_grpc::server::{Server, ServiceBuilder};

#[volo::main]
async fn main() {
  loginit::init();

  let addr: SocketAddr = match std::env::var("CLIP_SEARCH_TXT_GRPC") {
    Ok(uri) => uri,
    Err(_) => "0.0.0.0:8312".into(),
  }
  .parse()
  .unwrap();

  tracing::info!("grpc://{addr}");
  let addr = volo::net::Address::from(addr);

  Server::new()
    .add_service(ServiceBuilder::new(volo_gen::rpc::RpcServer::new(S)).build())
    .run(addr)
    .await
    .unwrap();
}
