#!/usr/bin/env coffee
> @w5/wasm > vbyteE vbyteD
  @w5/cid > CID_IMG
  @w5/redis/KV

bin = Buffer.from 'Ao_kHQ','base64url'
# id = 740446
# bin = Buffer.from vbyteE [CID_IMG,id]
#
key = 'r1'
console.log await KV.zscore key, bin
console.log await KV.zadd key, bin, 20000
