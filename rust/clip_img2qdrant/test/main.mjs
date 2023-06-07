#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
import test from 'ava';

import {
  Db
} from '../index.js';

test("clip", (t) => {
  console.log(Db('clip'));
  // db = new Dbx 'clip'
  // console.log db
  t.pass();
});
