// use std::str::FromStr;
//
use anyhow::anyhow;
use awp::Result;
use axum::{
  extract::Path,
  response::{IntoResponse, Response},
};
use reqwest::header::CONTENT_TYPE;

use crate::ONNX;

pub async fn get(Path(args): Path<String>) -> Result<Response> {
  if args == "favicon.ico" {
    return Ok("".into_response());
  }

  let url = unsafe { crate::env::TO.clone() + &args };
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
  return rt!(mime, bin);
}
