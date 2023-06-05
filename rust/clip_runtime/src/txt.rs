use std::sync::Arc;

use anyhow::Result;
use clip_txt::Tokener;
use ort::Environment;

use crate::{
  session::ClipSession,
  typedef::Arr,
  util::{box_iter_ndarray, box_u32_i64},
};

pub struct ClipTxt {
  pub env: Arc<Environment>,
  pub sess: ClipSession,
  pub tokener: Tokener,
}

impl ClipTxt {
  pub fn encode(&self, txt: impl AsRef<str>) -> Result<Arr> {
    let (ids, mask) = self.tokener.encode(txt)?;
    let ids = box_iter_ndarray([box_u32_i64(ids)].into_iter())?;
    let mask = box_iter_ndarray([box_u32_i64(mask)].into_iter())?;
    Ok(self.sess.run([ids, mask])?)
  }

  pub fn encode_batch(
    &self,
    txt_li: impl ExactSizeIterator + Iterator<Item = impl AsRef<str>>,
  ) -> Result<Arr> {
    let (ids_li, mask_li) = self.tokener.encode_batch(txt_li)?;
    let ids_li = box_iter_ndarray(ids_li.into_iter().map(box_u32_i64))?;
    let mask_li = box_iter_ndarray(mask_li.into_iter().map(box_u32_i64))?;
    Ok(self.sess.run([ids_li, mask_li])?)
  }
}

#[cfg(test)]
mod tests {
  use anyhow::Result;

  #[test]
  fn test() -> Result<()> {
    let model = crate::test::clip_model();

    let clip_txt = model.txt("onnx/Txt", 77)?;

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
