#!/usr/bin/env coffee

# myHeaders = new Headers()
# myHeaders.append("Content-Type", "application/json")
# myHeaders.append("api-key", "EoEHijputdjGnGuB_as8")
#
#
# requestOptions = {
#   method: 'PUT',
#   headers: myHeaders,
#   body: raw,
#   redirect: 'follow'
# }

console.log process.env.QDRANT_URL+process.env.QDRANT__SERVICE__HTTP_PORT
# fetch("http://localhost:3680/collections/test_collection", requestOptions)
#   .then(response => response.text())
#   .then(result => console.log(result))
#   .catch(error => console.log('error', error))
