#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var URL, hash;

import reqBin from '@w5/req/reqBin';

URL = 'https://5ok.pw/h950/';

hash = '824lDpfyaIFkmQd0D9Qldg';

console.log((await reqBin(URL + hash)));
