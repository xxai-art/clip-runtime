#!/usr/bin/env coffee
> @w5/wasm > u64Bin
  @w5/redis/KV

id = 740446
bin = u64Bin id

key = 'r1'
console.log await KV.zscore key, bin
