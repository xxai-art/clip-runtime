use lingua::Language::{
  self, Arabic, Chinese, Dutch, English, French, German, Hindi, Italian, Japanese, Korean, Polish,
  Portuguese, Russian, Spanish, Thai, Turkish, Ukrainian, Vietnamese,
};
use phf::phf_map;

pub static LANG_CODE: phf::Map<&'static str, usize> = phf_map! {
  "zh" => 0,
  "en" => 1,
  "ja" => 2,
  "th" => 3,
  "ko" => 4,
  "hi" => 5,
  "uk" => 6,
  "ar" => 7,
  "tr" => 8,
  "vi" => 9,
  "pl" => 10,
  "nl" => 11,
  "pt" => 12,
  "it" => 13,
  "es" => 14,
  "de" => 15,
  "fr" => 16,
  "ru" => 17
};

pub const LANG: [Language; 18] = [
  Chinese, English, Japanese, Thai, Korean, Hindi, Ukrainian, Arabic, Turkish, Vietnamese, Polish,
  Dutch, Portuguese, Italian, Spanish, German, French, Russian,
];
