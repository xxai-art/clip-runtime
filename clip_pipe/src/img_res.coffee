#!/usr/bin/env coffee

> @w5/redis/KV
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

prompt = """
masterpiece, best quality,photorealistic,best shadow, delicate, RAW,(1 girl,cute, black hair,very long hair),<lora:loong2:1>,(loong midair, flying,Looking at the camera,misty,fog, ), Dark clouds cover the sky,misty , cloudy sky (Sexy Chinese Taoist woman (long hair,Sexy Taoist robe ) ) , <lora:add_detail:0.6> ,<lora:style_masterpiece(0_7):0.3>
ng_deepnegative_v1_75t, easynegative,badhandv4, (worst quality:2), (low quality:2), (normal quality:2), lowres, ((monochrome)), ((grayscale)), watermark
"""

[embed,lora] = await prompt2res prompt
console.log { embed, lora }

process.exit()
