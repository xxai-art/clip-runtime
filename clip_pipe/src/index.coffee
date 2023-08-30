#!/usr/bin/env coffee

> @w5/redistream
  ./upload



并发 = (+ process.env.TASK_PRE_CPU) or 10

BEGIN = +new Date
N = 0

# await upload 215060
await redistream.clip(
  (id)=>
    await upload(id)
    console.log ((new Date - BEGIN) / ++N)/1000+' s / item'
    return true
  并发
)

process.exit()
