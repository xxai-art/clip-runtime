#!/usr/bin/env coffee
> @w5/uintbin/cidB64
  msgpackr > pack
  @w5/wasm > vbyteE u64Bin
  @w5/ossput
  @w5/cid > CID_IMG
  @w5/redis/KV
  @w5/pg/APG > ONE
  ./qdrant
  ./RES_META


export default (id)=>
  args = await ONE"SELECT adult,hash::bytea,rid,time,iaa,cid FROM bot.task WHERE id=#{id}"
  if not args
    return
  [adult,hash,rid,time,iaa] = args

  src_id = args.pop()
  switch src_id
    when 1 # 来源 https://civitai.com
      t = await ONE"SELECT star,w,h from bot.civitai_img WHERE id=#{rid}"
      if not t
        return
      [star,w,h] = t
      if h <= 0
        return
      w_h_r = Math.round(w * 1024 / h)
      star = Math.log1p(star or 0)*25
      iaa += Math.round star
    else
      return

  cid = CID_IMG
  score = 20000 + iaa

  adult = if adult > 0 then 1 else 0

  ing = []
  for [prefix,key] from [
    [
      'rec'
      vbyteE [cid, id, w_h_r]
    ]
  ]
    key = Buffer.from key
    for zset from [prefix, prefix+adult]
      ing.push KV.zadd zset, key, score
  await Promise.all [
    Promise.all ing
    do =>
      url = cidB64(cid,id)
      func = RES_META[cid]
      meta = await func id, ...args
      if meta
        await ossput(
          url
          => pack meta
          # https://developers.cloudflare.com/support/speed/optimization-file-size/what-will-cloudflare-compress/
          'text/x-script'
        )
      return
  ]
  day = Math.floor time/86400
  await qdrant id,hash,w_h_r,score,!adult,day
  return

