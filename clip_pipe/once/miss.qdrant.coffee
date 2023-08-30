#!/usr/bin/env coffee
> @w5/pg/APG > ITER ONE LI0 ONE0
  @w5/cid > CID_IMG
  @w5/wasm > vbyteE
  @w5/qdrant:Q
  @w5/redis/KV
  @w5/sleep
  @w5/redistream/XaddLi


limit = 9999
SAME = new Set()

# POST /collections/{collection_name}/points/delete
for await [id] from ITER.bot.clip_same('',{limit})
  SAME.add id

{clip:xadd} = XaddLi

{clip} = Q.POST.collections
{points} = clip
points_payload = points.payload

CID_MAP = new Map
CID_MAP.set 1, CID_IMG

hset = (cid,id,adult,rid,iaa,ing)=>
  p = do =>
    cid = CID_MAP.get(cid)
    star = await ONE0"SELECT star from bot.civitai_img WHERE id=#{rid}"
    star = Math.log1p(star or 0)*25
    score = Math.round(iaa+star)
    id_bin = Buffer.from vbyteE [cid, id]
    score += 20000
    for k from ['rec','rec'+(+!!adult),'']
      await KV.zadd k,id_bin,score
    await KV.zadd 'recImg', Buffer.from(u64Bin(id)), score
    ing.delete p
    return
  p

clip_iter = ->
  iter = ITER.bot.task('cid,rid,iaa,adult',{where:"iaa>25", limit, id:(+process.env.ID) or 0})
  m = new Map
  for await [id,cid,rid,iaa,adult] from iter
    if cid == 1
      if not SAME.has id
        m.set id,[cid,rid,iaa,adult]
        if m.size >= limit
          yield m
          m = new Map
  if m.size
    yield m

  return

miss = 0

ing = new Set()
for await m from clip_iter()
  ids = [...m.keys()]

  exist = new Set ids
  li = await points {
    ids
    with_payload:true
  }

  for i from li
    {id, payload} = i
    if 'w' of payload
      exist.delete id
      [cid,rid,iaa,adult] = m.get id
      ing.add hset cid,id,adult,rid,iaa,ing
      if ing.size > 100
        console.log 'ing',ing.size
        await Promise.all [...ing]
        console.log '>ing',ing.size

  {size} = exist
  if size
    miss += size
    exist = Array.from exist
    await xadd exist.map (i)=>[i]
    console.log 'miss',miss

if ing.size
  await Promise.all [...ing]
console.log 'done'
process.exit()
