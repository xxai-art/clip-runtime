#!/usr/bin/env coffee
> @w5/uintbin/cidB64
  msgpackr > pack
  @w5/wasm > vbyteE
  @w5/ossput
  @w5/cid > CID_IMG
  @w5/redis/KV
  @w5/pg/APG > ONE ONE0
  ./qdrant
  ./RES_META


export default (id)=>
  [adult,hash,rid,time,iaa] = args = await ONE"SELECT adult,hash::bytea,rid,time,iaa,cid FROM bot.task WHERE id=#{id}"

  cid = args.pop()
  if cid == 1 # 来源 https://civitai.com
    star = await ONE0"SELECT star from bot.civitai_img WHERE id=#{rid}"
    star = Math.log1p(star or 0)*25
    iaa += Math.round star
    cid = CID_IMG
  else
    return

  bin_id = Buffer.from vbyteE [cid, id]
  score = 20000 + iaa

  adult = if adult > 0 then 1 else 0

  rec = 'rec'
  await Promise.all [
    KV.zadd rec+adult,bin_id,score
    KV.zadd rec,bin_id,score
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
  await qdrant id,hash,adult,day
  return

