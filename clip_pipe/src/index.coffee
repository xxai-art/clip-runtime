#!/usr/bin/env coffee

> @w5/clip_img2qdrant > Db
  @w5/pg/PG > UPSERT
  @w5/redistream
  @w5/req/reqBin

PORN = await UPSERT('tag.tag',{val:'porn'})

URL = 'https://5ok.pw/h950/'
{clip:DB} = Db

await redistream.clip (id, hash, adult)=>
  hash = Buffer.from(hash).toString 'base64url'
  url = URL+hash
  console.log url
  bin = await reqBin url

  t = []
  if adult
    t.push PORN

  await DB.add(
    id
    {
      t
    }
    bin
    'avif'
  )

  return true


process.exit()
