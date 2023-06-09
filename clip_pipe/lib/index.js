#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
import reqBin from '@w5/req/reqBin';

import {
  Db
} from '@w5/clip_img2qdrant';

import KV from '@w5/redis/KV';

import R from '@w5/redis/R';

import gid from '@w5/gid';

console.log(R, gid, KV);

// URL = 'https://5ok.pw/h950/'

// {clip:DB} = Db

// hash = '824lDpfyaIFkmQd0D9Qldg'

// bin = await reqBin URL+hash

// await DB.add(
//   1
//   {
//     t: [1,3]
//   }
//   bin
//   'avif'
// )

