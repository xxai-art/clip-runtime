#!/usr/bin/env coffee

> ava:test
  @w5/clip > Model
  @w5/uridir
  path > dirname join

ROOT = dirname dirname uridir(import.meta)

test(
  'txt'
  (t)=>
    dir = join ROOT,'model',process.env.MODEL
    model = Model dir
    console.log model
    t.pass()
    return
)
