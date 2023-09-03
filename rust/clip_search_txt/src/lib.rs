#![feature(impl_trait_in_assoc_type)]
#![feature(async_fn_in_trait)]
#![feature(const_trait_impl)]
mod lang;
use std::sync::OnceLock;

use clip_qdrant::qdrant_client;
use clip_runtime::{ort::ClipOrt, txt::ClipTxt};
use qdrant_client::qdrant::{point_id::PointIdOptions, Condition, Filter, Range, SearchPoints};
use volo_gen::rpc::{DayRange, Level, Point, QIn, QOut, Rpc};
use volo_macro::volo;

pub static ONNX: OnceLock<ClipTxt> = OnceLock::new();
pub static CLIP: &'static str = "clip";
pub static SFW: &'static str = "sfw";

pub fn onnx() -> &'static ClipTxt {
  loop {
    match ONNX.get() {
      Some(r) => return r,
      None => {
        let ort = ClipOrt::new().unwrap();
        let model = ort.model(std::env::var("MODEL_DIR").unwrap());
        let onnx = model.txt("onnx/Txt", 77).unwrap();
        let _ = ONNX.set(onnx);
        continue;
      }
    }
  }
}

pub fn search_wh(must: &mut Vec<Condition>, w: u32, h: u32) {
  macro_rules! must_push {
    ($x:ident) => {
      if $x > 0 {
        must.push(Condition::range(
          stringify!($x),
          Range {
            gte: Some($x as _),
            ..Default::default()
          },
        ))
      }
    };
  }
  must_push!(w);
  must_push!(h);
}

pub fn search_day_range(day_range: DayRange) -> Condition {
  let begin = day_range.begin;
  let end = day_range.end;
  Condition::range(
    "day",
    Range {
      gte: Some(begin as _),
      lt: Some(end as _),
      ..Default::default()
    },
  )
}

async fn clip(msg: QIn) -> anyhow::Result<QOut> {
  let txt = msg.txt;
  let level = msg.level;
  let txt = lang::detect::prompt(txt, msg.lang);
  dbg!(&txt, &level);
  let vec = onnx().encode(txt)?.into_raw_vec();

  let client = qdrant_client();

  let (limit, offset) = if let Some(offset_limit) = msg.offset_limit {
    (offset_limit.limit as _, Some(offset_limit.offset as _))
  } else {
    (1024, None)
  };

  let mut point = SearchPoints {
    collection_name: CLIP.to_string(),
    vector: vec,
    limit,
    offset,
    with_payload: None, //Some(true.into()),
    ..Default::default()
  };

  let mut must = vec![];
  let mut must_not = vec![];

  if level == Level::Nsfw {
    must.push(Condition::matches(SFW, false));
  } else if level == Level::Sfw {
    must_not.push(Condition::is_empty(SFW))
  }

  if let Some(day_range) = msg.day_range {
    must.push(search_day_range(day_range));
  }

  search_wh(&mut must, msg.w, msg.h);

  if !(must.is_empty() && must_not.is_empty()) {
    let mut filter = Filter::default();
    if !must.is_empty() {
      filter.must = must;
    }
    if !must_not.is_empty() {
      filter.must_not = must_not;
    }
    point.filter = Some(filter);
  }

  let response = client.search_points(&point).await?.result;
  let li: Vec<_> = response
    .into_iter()
    .filter_map(|i| {
      if let PointIdOptions::Num(id) = i.id?.point_id_options? {
        Some(Point { id, score: i.score })
      } else {
        None
      }
    })
    .collect();

  Ok(QOut { li })
}

volo!(
    Rpc
    clip(self, req:QIn) -> QOut { clip( req.into_inner()).await? }
);
