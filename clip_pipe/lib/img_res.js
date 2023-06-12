#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var R_NAME_EMBED, R_NAME_LORA, kvGet, prompt2res, res_by_id;

import KV from '@w5/redis/KV';

import {
  ONE,
  ONE0,
  LI,
  Q
} from '@w5/pg/PG';

import binUint from '@w5/uintbin/binUint';

R_NAME_EMBED = 'nameEmbed';

R_NAME_LORA = 'nameLora';

kvGet = async(key, set) => {
  var _li, i, j, len, li, pos, ref;
  li = [];
  if (set.size) {
    _li = Array.from(set);
    ref = (await KV.hmgetB(key, _li));
    for (pos = j = 0, len = ref.length; j < len; pos = ++j) {
      i = ref[pos];
      if (i) {
        li.push([binUint(i), _li[pos]]);
      }
    }
  }
  return li;
};

prompt2res = (prompt) => {
  var embed_set, i, lora_set, p;
  prompt = prompt.toLocaleLowerCase().replaceAll('\n', ' ').replaceAll(')', ',').replaceAll('(', ',').replaceAll('[', ',').replaceAll(']', ',').replaceAll('>', ',').split(',').map((i) => {
    return i.trim();
  }).filter((i) => {
    return i;
  });
  lora_set = new Set();
  embed_set = new Set();
  for (i of prompt) {
    if (i.startsWith('<')) {
      p = i.indexOf(':', 1);
      if (p > 0) {
        lora_set.add(i.replaceAll('<', '').split(':')[1].trim());
        continue;
      }
      i = i.replaceAll('<', '').trim();
    }
    if (!i.includes(' ')) {
      embed_set.add(i);
    }
  }
  return Promise.all([kvGet(R_NAME_EMBED, embed_set), kvGet(R_NAME_LORA, lora_set)]);
};

res_by_id = async(id) => {
  var cid, embed, hash, i, id_set, li, lora, name, nprompt, nprompt_id, prompt, prompt_id, res_file_id_li, rid, txt, x, y;
  // [embed,lora] = await prompt2res prompt
  // console.log { embed, lora }
  [cid, rid, hash] = (await ONE`SELECT cid,rid,hash::bytea FROM bot.task WHERE id=${id}`);
  hash = hash.toString('base64url');
  switch (cid) {
    case 1:
      console.log({rid, hash});
      [prompt_id, nprompt_id, res_file_id_li] = (await ONE`SELECT prompt_id,nprompt_id,res_file_id_li FROM bot.civitai_img WHERE id=${rid}`);
      prompt = (await ONE0`SELECT val FROM img.prompt WHERE id=${prompt_id}`);
      nprompt = (await ONE0`SELECT val FROM img.nprompt WHERE id=${nprompt_id}`);
      txt = prompt + ',' + nprompt;
      console.log(txt);
      id_set = new Set(res_file_id_li);
      [embed, lora] = (await prompt2res(txt)); 
      for (x of embed) {
        [id, name] = x;
        id_set.add(id);
      }
      embed = new Map(embed);
      for (y of lora) {
        [id, name] = y;
        id_set.add(id);
      }
      lora = new Map(lora);
      console.log({embed, lora, res_file_id_li});
      console.log(id_set);
      if (id_set.size) {
        li = (await LI`SELECT sd.res_file.id,cid,sd.res_file.rid,sd.res.rid FROM sd.res_file,sd.res,sd.res_ver WHERE sd.res_file.id in ${Q([...id_set])} AND sd.res_ver.id=sd.res_file.res_ver_id AND sd.res.id=sd.res_file.res_id`);
        for (i of li) {
          console.log(i);
        }
      }
  }
};

console.log((await res_by_id(469472)));

process.exit();
