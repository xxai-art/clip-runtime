#!/usr/bin/env coffee

> ava:test
  @w5/clip > Clip clsImg
  glob-promise:glob
  fs > readFileSync
  @w5/uridir
  path > resolve join basename

ROOT = resolve uridir(import.meta),'../../..'

tmpl_kind_li = (prompt, ...li)=>
  r = []
  for i from li
    r.push prompt.replace('{}',i)
  r

test(
  'clip'
  (t)=>
    dir = join ROOT,'lib/model',process.env.MODEL
    clip = Clip dir

    context_length=77
    otxt = clip.txt 'onnx/Txt',context_length
    vec = otxt.encode 'a photo of cat'
    t.is vec.raw().length,1024
    word_li_li = [
      tmpl_kind_li("a photo of {}", "cat", "rat", "dog", "man", "woman"),
      tmpl_kind_li("一张{}的图片", "猫", "老鼠", "狗", "男人", "女人"),
    ]

    txt_feature_li = word_li_li.map otxt.encode.bind otxt

    oimg = clip.img 'onnx/Img',224
    for fp from await glob join(ROOT,'lib/img/*.jpg')
      img_feature = oimg.encode "jpg",readFileSync fp
      console.log basename fp
      for txt_feature, ti in txt_feature_li
        word_li = word_li_li[ti]
        for p, i in clsImg(txt_feature, img_feature)
          p = Math.round p*10000
          if p
            console.log word_li[i]+' '+(p/100)+'%'

    t.pass()
    return
)
