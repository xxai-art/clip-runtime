use std::collections::HashMap;

use lazy_static::lazy_static;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

use crate::lang::{
  prompt::PROMPT,
  var::{LANG, LANG_CODE},
};

lazy_static! {
  pub static ref LANG_POS: HashMap<Language, usize> = {
    let mut map = HashMap::new();
    for (pos, i) in LANG.iter().enumerate() {
      map.insert(*i, pos);
    }
    map
  };
  pub static ref LANG_DETECT: LanguageDetector = {
    tracing::info!("LOADING LANG DETECT");
    let lang_detect = LanguageDetectorBuilder::from_languages(&LANG)
      .with_preloaded_language_models()
      .build();
    tracing::info!("LOADED LANG DETECT");
    lang_detect
  };
}

pub fn detect(q: impl Into<String>, maybe: impl AsRef<str>) -> Option<usize> {
  let mut lang = None;
  let maybe_code = LANG_CODE.get(maybe.as_ref());
  for (language, confidence) in LANG_DETECT
    .compute_language_confidence_values(q)
    .into_iter()
  {
    if lang.is_none() {
      lang = LANG_POS.get(&language);
      if lang == maybe_code {
        break;
      }
    }
    if confidence < 0.01 {
      break;
    } else {
      let code = LANG_POS.get(&language);
      if code == maybe_code {
        lang = code;
        break;
      }
    }
  }
  lang.copied()
}

pub fn prompt(q: impl Into<String>, maybe: impl AsRef<str>) -> String {
  let q = q.into();
  if let Some(code) = detect(&q, maybe) {
    PROMPT[code].replace('#', &q)
  } else {
    q
  }
}
