#!/usr/bin/env coffee

> @w5/redistream
  ./upload


N = 0
BEGIN = +new Date

并发 = (+ process.env.TASK_PRE_CPU) or 1

N = 0

ING = new Set

pending = (id)=>
  await Promise.allSettled [...ING]
  p = upload id
  p.finally =>
    ING.delete(p)
    pending = upload
    return
  ING.add p
  await p
  return

# await upload 215060
await redistream.clip(
  (id)=>
    console.log id
    # 一个一个来，避免内存飙升被杀掉
    await pending id
    console.log ((new Date - BEGIN) / ++N)/1000+' s / item'
    return true
  并发
)

process.exit()
