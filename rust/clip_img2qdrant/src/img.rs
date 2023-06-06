use anyhow::anyhow;
use awp::Result;
use reqwest::header::CONTENT_TYPE;

use crate::{ONNX, Q};

pub async fn get(url: &str) -> Result<()> {
  let url = unsafe { crate::env::TO.clone() + url };
  dbg!(&url);
  let req = reqwest::get(&url).await?;

  let status = req.status();
  let mime = req
    .headers()
    .get(CONTENT_TYPE)
    .unwrap()
    .to_str()?
    .to_owned();

  let bin = req.bytes().await?;

  macro_rules! rt {
    ($content_type:expr, $content:expr) => {
      Ok((status, [(CONTENT_TYPE, $content_type)], $content).into_response())
    };
  }

  if status != 200 {
    return rt!(mime, bin);
  }

  let ext = if let Some(pos) = mime.rfind('/') {
    Some(&mime[1 + pos..])
  } else {
    None
  };
  let vec = ONNX.get().unwrap().encode(ext, &bin);
  let q = Q.get().unwrap();
  Ok(())
}
