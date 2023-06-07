#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
import test from 'ava';

import {
  Db
} from '../index.js';

test("clip", (t) => {
  var db;
  db = Db('clip');
  console.log(db);
  t.pass();
});
