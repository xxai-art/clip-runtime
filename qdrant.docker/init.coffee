#!/usr/bin/env coffee

> @w5/uridir
  @w5/read
  path > join basename
  glob-promise > glob

ROOT = uridir(import.meta)

URL = process.env.QDRANT_URL+':'+process.env.QDRANT__SERVICE__HTTP_PORT

HEADERS = new Headers()
HEADERS.append("Content-Type", "application/json")
HEADERS.append("api-key", process.env.QDRANT__SERVICE__API_KEY)

init = (fp)=>
  txt = read fp
  body = JSON.stringify JSON.parse txt
  db = basename(fp).slice(0,-5)
  console.log fp
  r = await fetch(
    URL+"/collections/"+db
    {
      method: 'PUT'
      headers: HEADERS
      redirect: 'follow'
      body
    }
  )
  {status} = await r.json()
  status


# fetch("http://localhost:3680/collections/test_collection", requestOptions)
#   .then(response => response.text())
#   .then(result => console.log(result))
#   .catch(error => console.log('error', error))



for db from await glob.glob join ROOT,"db/*.json"
  console.log await init(db)
