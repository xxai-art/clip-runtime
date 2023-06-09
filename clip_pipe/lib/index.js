#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var URL, bin, hash;

import reqBin from '@w5/req/reqBin';

URL = 'https://5ok.pw/h950/';

hash = '824lDpfyaIFkmQd0D9Qldg';

bin = (await reqBin(URL + hash));

1;

JSON.stringify({
  t: [1, 2, 3]
});
