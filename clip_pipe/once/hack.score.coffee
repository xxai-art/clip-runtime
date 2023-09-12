#!/usr/bin/env coffee
> @w5/wasm > vbyteE
  @w5/cid > CID_IMG
  @w5/redis/KV

id = 740446
bin = vbyteE [CID_IMG,id]

key = 'r1'
console.log await KV.zscore key, bin
