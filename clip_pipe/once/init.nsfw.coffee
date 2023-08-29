#!/usr/bin/env coffee

> @w5/pg/APG > ITER
  @w5/redis/KV
  @w5/wasm > u64Bin
  @w5/qdrant:Q

for await [id, nsfw] from ITER.bot.task('adult',{where:'adult>0'})

process.exit()
