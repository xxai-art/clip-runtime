#!/usr/bin/env coffee
> @w5/pg/APG > ITER ONE LI0 EXE Q
  @w5/qdrant:Qd
  @w5/sleep

{clip} = Qd.POST.collections
{points} = clip

limit = 999

to_small_iter = ->
  li = []
  for await [id,h] from ITER.bot.civitai_img('h',{limit,where:'h<512 or w<400'})
    li.push id
    if li.length >= limit
      yield li
      li = []
  if li.length
    yield li
  return

for await rid_li from to_small_iter()
  ids = await LI0"SELECT id FROM bot.task WHERE rid in #{Q rid_li}"
  li = await points {
    ids
    with_payload:true
  }
  if li.length
    o = {
      points : li.map (i)=>i.id
    }
    console.log o
    await points.delete o
    await EXE"DELETE FROM bot.civitai_img WHERE id in #{Q rid_li}"
process.exit()

