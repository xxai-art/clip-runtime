#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var I, T;

import avat from '@w5/avat';

I = (await import('../index.js'));

T = avat(I);

T.helloWorld([1, 2, 3])(3);
