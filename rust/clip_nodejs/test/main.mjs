#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var ROOT;

import test from 'ava';

import {
  Model
} from '@w5/clip';

import uridir from '@w5/uridir';

import {
  dirname,
  join
} from 'path';

ROOT = dirname(dirname(uridir(import.meta)));

test('txt', (t) => {
  var dir, model;
  dir = join(ROOT, 'model', process.env.MODEL);
  model = Model(dir);
  console.log(model);
  t.pass();
});
