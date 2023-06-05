#!/usr/bin/env coffee

> ava:test
  ../index.js > helloWorld

test(
  'main'
  (t)=>
    t.is(helloWorld([1,2,3]),3)
    return
)
