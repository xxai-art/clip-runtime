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
    otxt = clip.txt 'onnx/Txt',context_length
    vec = otxt.encode 'a photo of cat'
    t.is vec.raw().length,1024
    word_li = [
      'a photo of cat'
      'a photo of dog'
    ]
    vec_li = otxt.encode word_li
    n = 0
    for i from vec_li
      n++
      t.is i.length,1024
    t.is n,word_li.length


    t.pass()
    return
)
