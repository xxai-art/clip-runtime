#![feature(const_trait_impl)]

use std::net::SocketAddr;

use anyhow::Result;
use lazy_static::lazy_static;
pub use volo_gen::rpc::{DayRange, OffsetLimit, Point, QIn, QOut};
use volo_gen::rpc::{RpcClient, RpcClientBuilder};

lazy_static! {
  static ref CLIENT: RpcClient = {
    let addr: SocketAddr = std::env::var("CLIP_SEARCH_TXT_GRPC")
      .unwrap()
      .parse()
      .unwrap();
    RpcClientBuilder::new("rpc").address(addr).build()
  };
}

pub async fn clip(q: QIn) -> Result<QOut> {
  let resp = CLIENT.clip(q).await?;
  Ok(resp.into_inner())
}

#[cfg(test)]
mod tests {
  use anyhow::Result;

  use crate::{clip, OffsetLimit, QIn};

  #[tokio::test]
  async fn main() -> Result<()> {
    loginit::init();
    let txt = "狗".into();
    let req = QIn {
      txt,
      nsfw: -1,
      day_range: None, //Some(DayRange { begin: 0, end: 0 }),
      offset_limit: Some(OffsetLimit {
        offset: 0,
        limit: 10,
      }),
      lang: "zh".into(),
    };

    for i in clip(req).await?.li {
      // println!("{:?}", i)
      println!("{},", i.id);
    }

    Ok(())
  }
}
