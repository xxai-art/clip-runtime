#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var ROOT;

import test from 'ava';

import {
  Model
} from '@w5/clip';

import uridir from '@w5/uridir';

import {
  resolve,
  join
} from 'path';

ROOT = resolve(uridir(import.meta), '../../..');

test('txt', (t) => {
  var context_length, dir, model, otxt;
  dir = join(ROOT, 'lib/model', process.env.MODEL);
  model = new Model(dir);
  context_length = 77;
  console.log(model);
  otxt = model.txt('onnx/Txt', context_length);
  console.log(otxt);
  t.pass();
});
