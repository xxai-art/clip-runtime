#!/usr/bin/env coffee

> @w5/redistream/Xadd
  @w5/pg/APG > ITER LI0

{iaa,adult} = Xadd

scan = (xadd, key, where)=>
  set  = new Set

  for await key from ITER.bot.task(key,{where})
    p = xadd ...key
    console.log key
    set.add p
    p.then =>
      set.delete p
      return

  console.log 'wait',set.size

  await Promise.all(Array.from(set))
  return

await Promise.all [
  scan adult,'hash::bytea',"adult<0 AND hash IS NOT NULL"
  scan iaa,'',"adult>=0 AND iaa<0"
]
# await LI0"SELECT rid FROM bot.clip WHERE "
process.exit()
