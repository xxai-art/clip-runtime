use anyhow::Result;
use napi::JsNumber;
use napi_derive::napi;

#[napi]
pub fn hello_world(li: Vec<JsNumber>) -> Result<i64> {
  Ok(li.len() as i64)
}
