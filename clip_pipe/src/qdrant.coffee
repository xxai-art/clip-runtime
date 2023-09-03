#!/usr/bin/env coffee
> @w5/clip_img2qdrant > Db
  @w5/req/proxy
  @w5/req/reqBin
  @w5/pg/APG > EXE

{clip:DB} = Db

URL = 'https://5ok.pw/h952/'

< (id, hash, w_h_r, score, sfw, day)=>
  hash = Buffer.from(hash).toString 'base64url'
  url = URL+hash
  console.log url
  bin = await reqBin url, proxy
  o = {
    day
    sfw
    r:w_h_r
    s:score
  }
  rid = await DB.addIfNotExist(
    id
    JSON.stringify(o)
    bin
    'avif'
  )
  if rid
    rid = Number rid
    if rid != id
      await EXE"INSERT INTO bot.clip_same (id,rid) VALUES (#{id},#{rid}) ON CONFLICT (id) DO NOTHING"
  return
