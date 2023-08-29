use std::path::Path;

use anyhow::{anyhow, Result};
pub use tokenizers::tokenizer::Tokenizer;
pub struct Tokener {
  tokenizer: Tokenizer,
}

pub type VecIdsMask = (Vec<Box<[u32]>>, Vec<Box<[u32]>>);
pub type IdsMask = (Box<[u32]>, Box<[u32]>);

impl Tokener {
  pub fn from_file(path: impl AsRef<Path>, max_length: usize) -> Result<Self> {
    let mut tokenizer = Tokenizer::from_file(path).map_err(|e| anyhow!(e))?;
    let truncation = tokenizers::utils::truncation::TruncationParams {
      max_length,
      ..Default::default()
    };
    let _ = tokenizer.with_truncation(Some(truncation));
    Ok(Tokener { tokenizer })
  }

  pub fn encode_batch(
    &self,
    txt_li: impl ExactSizeIterator + Iterator<Item = impl AsRef<str>>,
  ) -> Result<VecIdsMask> {
    let len = txt_li.len();
    let mut id_li_li = Vec::with_capacity(len);
    let mut mask_li = Vec::with_capacity(len);
    if len > 0 {
      let tokenizer = &self.tokenizer;
      let li = tokenizer
        .encode_batch(
          txt_li.map(|x| x.as_ref().to_owned()).collect::<Vec<_>>(),
          true,
        )
        .map_err(|e| anyhow!(e))?;
      let max = li.iter().map(|item| item.get_ids().len()).max().unwrap();
      for encoding in li {
        let id_li = encoding.get_ids();
        let mask = encoding.get_attention_mask();

        let diff = max - id_li.len();
        let (id_li, mask) = if diff == 0 {
          (Box::from(id_li), Box::from(mask))
        } else {
          let mut id_li = id_li.to_vec();
          let mut mask = mask.to_vec();
          id_li.extend(std::iter::repeat(0).take(diff));
          mask.extend(std::iter::repeat(0).take(diff));
          (id_li.into(), mask.into())
        };

        id_li_li.push(id_li);
        mask_li.push(mask);
      }
    }
    Ok((id_li_li, mask_li))
  }

  pub fn encode(&self, txt: impl AsRef<str>) -> Result<IdsMask> {
    let tokenizer = &self.tokenizer;
    let encoding = tokenizer
      .encode(txt.as_ref(), true)
      .map_err(|e| anyhow!(e))?;
    let id_li = Box::from(encoding.get_ids());
    let mask = Box::from(encoding.get_attention_mask());
    Ok((id_li, mask))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() -> Result<()> {
    let mut process_dir: std::path::PathBuf = std::env::var("MODEL_DIR").unwrap().into();
    process_dir.push("process");
    let mut process_tokenizer_json = process_dir.clone();
    process_tokenizer_json.push("tokenizer.json");
    let tokener = Tokener::from_file(process_tokenizer_json, 77)?;
    let li = [
      "a photo of dog",
      "a photo of chinese woman",
      //"房贷利率（mortgage rate），是指在银行办理的用于购房的贷款，该贷款要按照银行规定的利率支付利息。中国房贷利率是由中国人民银行统一规定的，各个商业银行执行的时候可以在一定的区间内自行浮动；房贷利率不是一直不变的，而是经常变动的。",
    ];
    for word in li {
      let vec = tokener.encode(word)?;
      println!("\n❯ {}\n{:?}\n", word, vec);
    }
    // dbg!(tokener.encode_batch(li.into())?);
    Ok(())
  }
}
