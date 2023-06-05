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
  var clip, context_length, dir, i, n, otxt, vec, vec_li, word_li;
  dir = join(ROOT, 'lib/model', process.env.MODEL);
  clip = Clip(dir);
  context_length = 77;
  otxt = clip.txt('onnx/Txt', context_length);
  vec = otxt.encode('a photo of cat');
  t.is(vec.raw().length, 1024);
  word_li = ['a photo of cat', 'a photo of dog'];
  vec_li = otxt.encode(word_li);
  n = 0;
  for (i of vec_li) {
    n++;
    t.is(i.length, 1024);
  }
  t.is(n, word_li.length);
  t.pass();
});
