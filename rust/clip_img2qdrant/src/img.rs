use anyhow::Result;
use reqwest::header::CONTENT_TYPE;
use thiserror::Error;

use crate::ONNX;

#[derive(Error, Debug)]
pub enum NetError {
  #[error("{status} {url} → {body}")]
  Req {
    url: String,
    status: u16,
    body: String,
  },
}

pub async fn get(url: &str) -> Result<Vec<f32>> {
  let url = unsafe { crate::env::TO.clone() + url };
  let req = reqwest::get(&url).await?;

  let status = req.status();
  let mime = req
    .headers()
    .get(CONTENT_TYPE)
    .unwrap()
    .to_str()?
    .to_owned();

  let body = req.bytes().await?;

  if status != 200 {
    Err(NetError::Req {
      url,
      status: status.into(),
      body: String::from_utf8_lossy(&body).into(),
    })?;
  }

  let ext = mime.rfind('/').map(|pos| &mime[1 + pos..]);
  let vec = ONNX.get().unwrap().encode(ext, &body)?;
  Ok(vec.into_raw_vec())
}
