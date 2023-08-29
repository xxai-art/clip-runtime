#!/usr/bin/env coffee

> @w5/redis/AK
  @w5/uintbin/binUint

R_NAME_EMBED = 'nameEmbed'
R_NAME_LORA = 'nameLora'

kvGet = (key, set)=>
  li = []
  if set.size
    _li = Array.from(set).map (i)=>i.toLocaleLowerCase()
    for i,pos in await AK.hmgetB(key,_li)
      if i
        li.push [
          binUint i
          _li[pos]
        ]
  return li

< (prompt)=>
  prompt = prompt
    .replace(/[\r\n\s]+/g,' ')
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

    if i.includes ' '
      for j from i.split(' ')
        if j and (j.includes('-') or j.includes('_') or j.toLocaleLowerCase()!=j)
          embed_set.add j
    else
      embed_set.add i

  Promise.all [
    kvGet R_NAME_EMBED, embed_set
    kvGet R_NAME_LORA, lora_set
  ]



