#!/usr/bin/env coffee

> ava:test
  ../index.js > Db


test "clip",(t)=>
  console.log Db 'clip'
  # db = new Dbx 'clip'
  # console.log db
  t.pass()
  return

