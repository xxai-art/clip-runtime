#!/usr/bin/env coffee

> @w5/redis/KV
  @w5/pg/PG > ONE ONE0 LI Q
  @w5/uintbin/binUint

R_NAME_EMBED = 'nameEmbed'
R_NAME_LORA = 'nameLora'

kvGet = (key, set)=>
  li = []
  if set.size
    _li = Array.from set
    for i,pos in await KV.hmgetB(key,_li)
      if i
        li.push [
          binUint i
          _li[pos]
        ]
  return li

prompt2res = (prompt)=>
  prompt = prompt.toLocaleLowerCase()
    .replaceAll('\n',' ')
    .replaceAll(')',',')
    .replaceAll('(',',')
    .replaceAll('[',',')
    .replaceAll(']',',')
    .replaceAll('>',',')
    .split(',').map(
      (i)=>
        i.trim()
    ).filter((i)=>i)

  lora_set = new Set
  embed_set = new Set
  for i from prompt
    if i.startsWith('<')
      p = i.indexOf(':',1)
      if p > 0
        lora_set.add i.replaceAll('<','').split(':')[1].trim()
        continue
      i = i.replaceAll('<','').trim()

    if not i.includes(' ')
      embed_set.add i

  Promise.all [
    kvGet R_NAME_EMBED, embed_set
    kvGet R_NAME_LORA, lora_set
  ]

TEXTUALINVERSION = 4
LORA = new Set [1,3,6,TEXTUALINVERSION]

res_by_id = (id)=>
  # [embed,lora] = await prompt2res prompt
  # console.log { embed, lora }
  [cid,rid,hash] = await ONE"SELECT cid,rid,hash::bytea FROM bot.task WHERE id=#{id}"
  hash = hash.toString('base64url')
  switch cid
    when 1
      r = await ONE"SELECT prompt_id,nprompt_id,res_file_id_li,user_id,rid,sampler_id,w,h,step,genway_id,seed,time FROM bot.civitai_img WHERE id=#{rid}"
      [
        prompt_id
        nprompt_id
        res_file_id_li
      ] = r
      prompt = await ONE0"SELECT val FROM img.prompt WHERE id=#{prompt_id}"
      nprompt = await ONE0"SELECT val FROM img.nprompt WHERE id=#{nprompt_id}"
      txt = prompt+','+nprompt
      id_set = new Set res_file_id_li
      [embed, lora] = await prompt2res txt #

      for [id,name] from embed
        id_set.add id

      embed = new Map embed

      for [id, name] from lora
        id_set.add id

      lora = new Map lora

      if id_set.size
        li = Array.from await LI"SELECT cid,sd.res_file.id,kind_id,sd.res.name,sd.res_ver.rid,sd.res.rid FROM sd.res_file,sd.res,sd.res_ver WHERE sd.res_file.id in #{Q [...id_set]} AND sd.res_ver.id=sd.res_file.res_ver_id AND sd.res.id=sd.res_file.res_id"
        for i from li
          kind = i[2]
          if LORA.has kind
            if kind == TEXTUALINVERSION
              m = embed
            else
              m = lora
            key = m.get i[1]
            if key
              i.push key
      else
        li = []
      r[2] = li
      r.unshift nprompt
      r.unshift prompt
      r.unshift cid
      console.log r
  return


console.log await res_by_id(469472)
process.exit()
