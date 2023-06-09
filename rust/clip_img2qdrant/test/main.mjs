#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var DB, ROOT;

import test from 'ava';

import {
  Db
} from '../index.js';

import uridir from '@w5/uridir';

import {
  join,
  dirname
} from 'path';

import {
  readFileSync
} from 'fs';

ROOT = dirname(dirname(dirname(uridir(import.meta))));

DB = Db.clip;

test("clip", async(t) => {
  var fp, img;
  //fp = join ROOT,'lib/img/cat.jpg'
  fp = '/Users/z/art/clip-runtime/rust/clip_img2qdrant/1.avif';
  img = readFileSync(fp);
  console.log((await DB.add(1, {
    t: [1, 2, 3]
  }, img, "avif")));
  t.pass();
});
