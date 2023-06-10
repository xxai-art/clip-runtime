#!/usr/bin/env coffee

> @w5/redis/KV
  @w5/pg/PG > ONE
  @w5/uintbin/binUint

R_NAME_EMBED = 'nameEmbed'
R_NAME_LORA = 'nameLora'

kvGet = (key, set)=>
  _li = Array.from set
  li = []
  for i,pos in await KV.hmgetB(key,_li)
    if i
      li.push [
        _li[pos]
        binUint i
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

res_by_id = (id)=>
  # [embed,lora] = await prompt2res prompt
  # console.log { embed, lora }
  [cid,rid,hash] = await ONE"SELECT cid,rid,hash::bytea FROM bot.task WHERE id=#{id}"
  hash = hash.toString('base64url')
  switch cid
    when 1
      console.log {rid,hash}
      console.log await ONE"SELECT res_file_id_li FROM bot.civitai_img WHERE id=#{rid}"
      # await ONE"SELECT "

  return


console.log await res_by_id(469495)
process.exit()
