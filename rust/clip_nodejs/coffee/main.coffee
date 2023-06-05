#!/usr/bin/env coffee

> ava:test
  @w5/clip > Model
  @w5/uridir
  path > resolve join

ROOT = resolve uridir(import.meta),'../../..'

test(
  'txt'
  (t)=>
    dir = join ROOT,'lib/model',process.env.MODEL
    model = new Model dir
    context_length=77
    console.log model
    otxt = model.txt 'onnx/Txt',context_length
    console.log otxt
    t.pass()
    return
)
