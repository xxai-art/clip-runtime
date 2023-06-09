#!/usr/bin/env coffee

> @w5/req/reqBin

URL = 'https://5ok.pw/h950/'
hash = '824lDpfyaIFkmQd0D9Qldg'

bin = await reqBin URL+hash

1

JSON.stringify {
  t:[1,2,3]
}
