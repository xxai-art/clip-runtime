#!/usr/bin/env coffee

{
  QDRANT__SERVICE__HTTP_PORT
  QDRANT_URL
  QDRANT__SERVICE__API_KEY
} = process.env

headers = new Headers
headers.append 'api-key', QDRANT__SERVICE__API_KEY
headers.append 'Content-Type', 'application/json'

raw = JSON.stringify(
  'ids': [ 9 ]
  'with_payload': true
  # 'with_vectors': false
)

requestOptions =
  method: 'POST'
  headers: headers
  body: raw
  redirect: 'follow'

url = "#{QDRANT_URL}:#{QDRANT__SERVICE__HTTP_PORT}/collections/clip/points"

console.log url

console.log (await (await fetch(url, requestOptions)).json()).result

process.exit()
