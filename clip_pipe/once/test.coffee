#!/usr/bin/env coffee

> @w5/pg/APG > ITER LI0

limit = 9999
iter = ITER.bot.task('rid,iaa,cid',{where:"iaa>25", limit})
n = 0
for await [id,rid,iaa,cid] from iter
  n += 1
console.log n
