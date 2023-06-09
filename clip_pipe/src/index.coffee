#!/usr/bin/env coffee

> @w5/req/reqBin
  @w5/clip_img2qdrant > Db
  @w5/redis/KV
  @w5/redis/R
  @w5/gid

console.log R,gid,KV
# URL = 'https://5ok.pw/h950/'
#
# {clip:DB} = Db
#
# hash = '824lDpfyaIFkmQd0D9Qldg'
#
# bin = await reqBin URL+hash
#
# await DB.add(
#   1
#   {
#     t: [1,3]
#   }
#   bin
#   'avif'
# )
#
