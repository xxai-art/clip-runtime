#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var DB, URL, bin, hash;

import reqBin from '@w5/req/reqBin';

import {
  Db
} from '@w5/clip_img2qdrant';

URL = 'https://5ok.pw/h950/';

({
  clip: DB
} = Db);

hash = '824lDpfyaIFkmQd0D9Qldg';

bin = (await reqBin(URL + hash));

await DB.add(1, {
  t: [1, 3]
}, bin, 'avif');

// > ava:test
//   ../index.js > Db
//   @w5/uridir
//   path > join dirname
//   fs > readFileSync

// ROOT = dirname dirname dirname uridir import.meta

// DB = Db.clip

// test "clip",(t)=>
//   fp = join ROOT,'lib/img/cat.jpg'
//   img = readFileSync fp
//   console.log await DB.add(
//     1
//     {
//       t:[1,2,3]
//     }
//     img
//     "jpg"
//   )
//   t.pass()
//   return
