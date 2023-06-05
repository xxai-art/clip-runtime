#!/usr/bin/env coffee

> ava:test
  @w5/clip > Clip
  @w5/uridir
  path > resolve join

ROOT = resolve uridir(import.meta),'../../..'

test(
  'txt'
  (t)=>
    dir = join ROOT,'lib/model',process.env.MODEL
    clip = Clip dir
    context_length=77
    console.log clip
    otxt = clip.txt 'onnx/Txt',context_length
    vec = otxt.encode 'a photo of cat'
    vec_li = otxt.encode [
      'a photo of cat'
      'a photo of dog'
    ]
    console.log vec_li.len()+''
    # console.log JSON.stringify vec.get()
    t.pass()
    return
)
