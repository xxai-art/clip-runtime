#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var ROOT;

import test from 'ava';

import {
  Clip
} from '@w5/clip';

import uridir from '@w5/uridir';

import {
  resolve,
  join
} from 'path';

ROOT = resolve(uridir(import.meta), '../../..');

test('txt', (t) => {
  var clip, context_length, dir, otxt, vec, vec_li;
  dir = join(ROOT, 'lib/model', process.env.MODEL);
  clip = Clip(dir);
  context_length = 77;
  console.log(clip);
  otxt = clip.txt('onnx/Txt', context_length);
  vec = otxt.encode('a photo of cat');
  vec_li = otxt.encode(['a photo of cat', 'a photo of dog']);
  console.log(vec_li.len() + '');
  // console.log JSON.stringify vec.get()
  t.pass();
});
