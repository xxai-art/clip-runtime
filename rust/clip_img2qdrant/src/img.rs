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
// use crate::{img, img::Ext};
//
// const CONTENT_TYPE: &str = "content-type";
//
// pub fn byte_u32(bin: &[u8]) -> u32 {
//   let mut n = 0;
//   for b in bin.iter() {
//     n *= 10;
//     n += u32::from(b - b'0');
//   }
//   n
// }
//
// pub async fn proxy(id: &str, to_width: u32, to_height: u32) -> Result<Response> {
//   let hash;
//   let ext;
//
//   if let Some(pos) = id.rfind('.') {
//     hash = id[..pos].to_owned();
//     ext = Ext::from_str(&id[pos + 1..])?;
//   } else {
//     hash = id.into();
//     ext = Ext::avif;
//   }
//
//   let url = unsafe { crate::env::TO.clone() + &hash };
//   let req = reqwest::get(&url).await?;
//
//   let status = req.status();
//   let mime = req
//     .headers()
//     .get(CONTENT_TYPE)
//     .unwrap()
//     .to_str()?
//     .to_owned();
//
//   macro_rules! rt {
//     ($content_type:expr, $content:expr) => {
//       Ok((status, [(CONTENT_TYPE, $content_type)], $content).into_response())
//     };
//   }
//
//   let body = req.bytes().await?;
//
//   if status == 200 {
//     if let Some(c) = img::resize(&body, to_width, to_height, &mime, &ext)? {
//       let content_type = "image/".to_owned() + ext.as_ref();
//       return rt!(content_type, c);
//     }
//   }
//
//   rt!(mime, body)
// }

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
  dbg!(ext);
  let vec = ONNX.get().unwrap().encode(ext, &bin);
  dbg!(vec);
  return rt!(mime, bin);

  // let pos = pos.unwrap();
  // let opt = &args[..pos];
  // let opt = opt.split('-');
  // let id = &args[pos + 1..];
  //
  // let mut to_width = 0;
  // let mut to_height = 0;
  //
  // for i in opt {
  //   if i.len() >= 2 {
  //     let i = i.as_bytes();
  //     if i[1..].iter().all(|&byte| byte.is_ascii_digit()) {
  //       let start = i[0];
  //       let n = byte_u32(&i[1..]);
  //       if start == b'w' {
  //         to_width = n;
  //       } else if start == b'h' {
  //         to_height = n;
  //       }
  //     }
  //   }
  // }
  // proxy(id, to_width, to_height).await
}
