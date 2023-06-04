use std::sync::Arc;

use clip_txt::Tokener;
use ndarray::{ArrayBase, Dim, IxDynImpl, OwnedRepr};
use ort::{Environment, Session};

use crate::util::{box_iter_ndarray, box_u32_i64};

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
    let ids_li = box_iter_ndarray(ids_li.into_iter().map(box_u32_i64))?;
    let mask_li = box_iter_ndarray(mask_li.into_iter().map(box_u32_i64))?;
    let out = &self.sess.run([ids_li, mask_li])?;
    Ok(out[0].try_extract::<f32>()?.view().to_owned())
  }
}

#[cfg(test)]
mod tests {
  use clip_txt::Result;

  use crate::ort::ClipOrt;

  #[test]
  fn test() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut dir = std::env::current_dir()?;
    dir.push("model/AltCLIP-XLMR-L-m18");

    let ort = ClipOrt::new()?;

    let model = ort.model(dir.display().to_string());

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
