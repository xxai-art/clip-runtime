#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var DB, PORN, URL;

import {
  Db
} from '@w5/clip_img2qdrant';

import gid from '@w5/gid';

import {
  UPSERT
} from '@w5/pg/PG';

import KV from '@w5/redis/KV';

import R from '@w5/redis/R';

import redistream from '@w5/redistream';

import reqBin from '@w5/req/reqBin';

PORN = (await UPSERT('tag.tag', {
  val: 'porn'
}));

URL = 'https://5ok.pw/h950/';

({
  clip: DB
} = Db);

await redistream.clip(async(id, hash, adult) => {
  var bin, t, url;
  hash = Buffer.from(hash).toString('base64url');
  url = URL + hash;
  console.log(url);
  bin = (await reqBin(url));
  t = [];
  if (adult) {
    t.push(PORN);
  }
  await DB.add(id, {t}, bin, 'avif');
  return true;
});

process.exit();
