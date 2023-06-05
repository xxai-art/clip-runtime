#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
import test from 'ava';

import {
  helloWorld
} from '../index.js';

test('main', (t) => {
  t.is(helloWorld([1, 2, 3]), 3);
});
