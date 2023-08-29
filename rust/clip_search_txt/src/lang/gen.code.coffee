#!/usr/bin/env coffee

> @w5/uridir
  @w5/write
  path > join

ROOT = uridir import.meta

LI = """ZH 简体中文 Chinese
EN 英语 English
JA 日语 Japanese
TH 泰语 Thai
KO 韩语 Korean
HI 印地语 Hindi
UK 乌克兰语 Ukrainian
AR 阿拉伯语 Arabic
TR 土耳其语 Turkish
VI 越南语 Vietnamese
PL 波兰语 Polish
NL 荷兰语 Dutch
PT 葡萄牙语 Portuguese
IT 意大利语 Italian
ES 西班牙语 Spanish
DE 德语 German
FR 法语 French
RU 俄语 Russian""".split('\n').map (i)=>
  i.split(' ')

enum_lang = []
lang_code = []
lang_li = []

for [en,cn,lang],pos in LI
  enum_lang.push "#{en} = #{pos}, // #{cn}"
  lang_code.push "\"#{en.toLowerCase()}\" => #{pos}"
  lang_li.push lang

# #[derive(Copy, Clone)]
# pub enum Lang {
#   #{enum_lang.join('\n  ')}
# }

RS = """
use phf::phf_map;
use lingua::Language::{self,#{lang_li.join(',')}};


pub static LANG_CODE: phf::Map<&'static str, usize> = phf_map! {
  #{lang_code.join(",\n  ")}
};

pub const LANG: [Language; #{LI.length}] = [
  #{lang_li.join(',\n  ')}
];
"""

write(
  join ROOT,'var.rs'
  RS
)
