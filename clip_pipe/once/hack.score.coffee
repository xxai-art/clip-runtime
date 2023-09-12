#!/usr/bin/env coffee
> @w5/wasm > u64Bin
  @w5/redis/KV

id = 740446
bin = u64Bin id

console.log await KV.zscore bin
