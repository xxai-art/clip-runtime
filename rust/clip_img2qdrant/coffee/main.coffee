#!/usr/bin/env coffee

> ava:test
  ../index.js > Db
  @w5/uridir
  path > join dirname
  fs > readFileSync

ROOT = dirname dirname dirname uridir import.meta

DB = Db.clip

test "clip",(t)=>
  fp = join ROOT,'lib/img/cat.jpg'
  img = readFileSync fp
  console.log await DB.add(
    1
    {
      t:[1,2,3]
    }
    img
    "jpg"
  )
  t.pass()
  return

