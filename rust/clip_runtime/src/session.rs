use std::{ops::Deref, sync::Arc};

use anyhow::Result;
use ndarray::Ix2;
use ort::{environment::Environment, GraphOptimizationLevel, Session, SessionBuilder, Value};

use crate::typedef::Arr;

pub struct ClipSession(Session);

impl Deref for ClipSession {
  type Target = Session;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[macro_export]
macro_rules! run {
   ($sess:expr,$($i:ident),*) => {{
      let sess = &$sess;
      let allocator = sess.allocator();
      $(
        let $i = ndarray::CowArray::from($i.into_dyn());
      )*
      let r = sess.run(
      vec![$(
        ort::Value::from_array(allocator,&$i)?,
      )*]
      );
      r
   }};
}

impl ClipSession {
  pub fn new(dir: &str, env: &Arc<Environment>, onnx: &str) -> Result<Self> {
    Ok(Self(
      SessionBuilder::new(env)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_inter_threads(num_cpus::get() as i16)?
        .with_model_from_file(format!("{}/{}.onnx", dir, onnx))?,
    ))
  }

  pub fn run(&self, li: Vec<Value<'_>>) -> anyhow::Result<Arr> {
    let out = self.0.run(li)?;
    Ok(
      out[0]
        .try_extract::<f32>()?
        .view()
        .to_owned()
        .into_dimensionality::<Ix2>()?,
    )
  }
}
