> @w5/pg/APG > ONE ONE0 LI Q
  @w5/cid > CID_IMG
  ./prompt2res

CHECKPOINT = 2
TEXTUALINVERSION = 4
LORA = new Set [1,3,6,TEXTUALINVERSION]

export default RES_META = {}

RES_META[CID_IMG] = (id,adult,hash,rid,time)=>
  # [embed,lora] = await prompt2res prompt
  # console.log { embed, lora }
  r = await ONE"SELECT prompt_id,nprompt_id,res_file_id_li,user_id,sampler_id,w,h,step,genway_id,seed,rid,cfg FROM bot.civitai_img WHERE id=#{rid}"
  [
    prompt_id
    nprompt_id
    res_file_id_li
  ] = r
  prompt = await ONE0"SELECT val FROM img.prompt WHERE id=#{prompt_id}"
  nprompt = await ONE0"SELECT val FROM img.nprompt WHERE id=#{nprompt_id}"
  txt = prompt+','+nprompt
  id_set = new Set res_file_id_li
  [embed, lora] = await prompt2res txt

  for [id,name] from embed
    id_set.add id

  embed = new Map embed

  for [id, name] from lora
    id_set.add id

  lora = new Map lora

  if id_set.size
    li = Array.from await LI"SELECT sd.res_file.id,cid,kind_id,sd.res.name,sd.res_ver.rid,sd.res.rid FROM sd.res_file,sd.res,sd.res_ver WHERE sd.res_file.id in #{Q [...id_set]} AND sd.res_ver.id=sd.res_file.res_ver_id AND sd.res.id=sd.res_file.res_id"
    for i from li
      key = i.shift()
      kind = i[1]
      if LORA.has kind
        if kind == TEXTUALINVERSION
          m = embed
        else
          m = lora
        key = m.get key
        if key
          if key == i[2].toLocaleLowerCase()
            key = 0
            # 相同名称的时候，以空字符串占位
          i.push key
  else
    li = []

  if li.length == 0
    li = 0

  r[2] = li
  if r[11] # 有 rid 的时候才显示 user
    user_id = r[3]
    if user_id
      r[3] = [
        r[3]
        await ONE0"SELECT val FROM bot.civitai_user WHERE id=#{user_id}"
      ]
  else
    r[3] = 0
  return [
    time
    adult
    hash
    prompt
    nprompt
  ].concat r
