#!/usr/bin/env coffee

> ava:test
  ../index.js > Db


test "clip",(t)=>
  db = Db 'clip'
  console.log db
  t.pass()
  return

