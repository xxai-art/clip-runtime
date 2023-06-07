#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
import test from 'ava';

test("clip", (t) => {
  t.pass();
});
