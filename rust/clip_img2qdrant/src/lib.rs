use anyhow::Result;
use napi::JsNumber;
use napi_derive::napi;

#[napi]
pub fn hello_world(li: Vec<JsNumber>) -> Result<i64> {
  Ok(li.len() as i64)
}

use napi::{
  bindgen_prelude::{AsyncTask, Buffer},
  Env, Task,
};

pub struct AsyncArg(i64, Buffer);

impl Task for AsyncArg {
  type Output = i64;
  type JsValue = i64;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(self.0 + self.1.len() as i64)
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn async_add(a: i64, b: Buffer) -> AsyncTask<AsyncArg> {
  AsyncTask::new(AsyncArg(a, b))
}
