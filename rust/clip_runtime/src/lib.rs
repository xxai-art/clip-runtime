pub mod providers;
use std::{path::Path, sync::Arc};

use anyhow::Result;
use clip_txt::Tokener;
use ndarray::{Array2, ArrayBase, Dim, IxDynImpl, OwnedRepr};
use ort::{
  environment::Environment,
  tensor::{FromArray, InputTensor},
  GraphOptimizationLevel, Session, SessionBuilder,
};

use crate::providers::providers;

pub fn box_u32_i64(li: Box<[u32]>) -> Vec<i64> {
  li.into_vec().into_iter().map(|x| x as i64).collect()
}

pub fn box_iter_ndarray<T: std::clone::Clone>(
  boxli: impl ExactSizeIterator + Iterator<Item = Vec<T>>,
) -> anyhow::Result<InputTensor>
where
  InputTensor: FromArray<T>,
{
  let mut vec = Vec::new();
  let len = boxli.len();
  for b in boxli {
    vec.extend_from_slice(&b);
  }
  Ok(InputTensor::from_array(
    Array2::from_shape_vec((len, vec.len() / len), vec)?.into_dyn(),
  ))
}

pub struct ClipOrt {
  pub env: Arc<Environment>,
}

impl ClipOrt {
  pub fn new() -> Result<Self> {
    Ok(ClipOrt {
      env: Arc::new(
        Environment::builder()
          .with_name("clip")
          .with_execution_providers(providers())
          .build()?,
      ),
    })
  }
  pub fn txt(
    &self,
    dir: impl AsRef<Path>,
    onnx: &str,
    context_length: usize,
  ) -> clip_txt::Result<ClipTxt> {
    let env = self.env.clone();
    let dir = dir.as_ref().display();
    let sess = SessionBuilder::new(&env)?
      .with_optimization_level(GraphOptimizationLevel::Level3)?
      .with_inter_threads(num_cpus::get() as i16)?
      .with_model_from_file(format!("{}/{}.onnx", dir, onnx))?;
    Ok(ClipTxt {
      env,
      sess,
      tokener: Tokener::from_file(format!("{}/process/tokenizer.json", dir), context_length)?,
    })
  }
}

pub struct ClipTxt {
  pub env: Arc<Environment>,
  pub sess: Session,
  pub tokener: Tokener,
}

impl ClipTxt {
  pub fn encode(
    &self,
    txt: impl AsRef<str>,
  ) -> clip_txt::Result<ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>> {
    let (ids, mask) = self.tokener.encode(txt)?;
    let ids = box_iter_ndarray([box_u32_i64(ids)].into_iter())?;
    let mask = box_iter_ndarray([box_u32_i64(mask)].into_iter())?;
    let out = &self.sess.run([ids, mask])?;
    Ok(out[0].try_extract::<f32>()?.view().to_owned())
  }

  pub fn encode_batch(
    &self,
    txt_li: impl ExactSizeIterator + Iterator<Item = impl AsRef<str>>,
  ) -> clip_txt::Result<ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>> {
    let (ids_li, mask_li) = self.tokener.encode_batch(txt_li)?;
    let ids_li = box_iter_ndarray(ids_li.into_iter().map(|i| box_u32_i64(i)))?;
    let mask_li = box_iter_ndarray(mask_li.into_iter().map(|i| box_u32_i64(i)))?;
    let out = &self.sess.run([ids_li, mask_li])?;
    Ok(out[0].try_extract::<f32>()?.view().to_owned())
  }
}

#[cfg(test)]
mod tests {
  use clip_txt::Result;

  use super::*;

  #[test]
  fn test() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut dir = std::env::current_dir()?;
    dir.push("model/AltCLIP-XLMR-L-m18");

    let ort = ClipOrt::new()?;
    let clip_txt = ort.txt(dir, "onnx/Txt", 77)?;
    let word_li = [
      "a photo for chinese woman",
      "a photo of dog",
      "a photo for cat",
    ];
    for word in word_li {
      let out = clip_txt.encode(word)?;
      println!("❯ {}\n{}\n", word, out);
    }
    let out = clip_txt.encode_batch(word_li.into_iter())?;
    for (out, word) in out.rows().into_iter().zip(word_li.iter()) {
      println!("❯ {}\n{}\n", word, out);
    }
    Ok(())
  }
}
