#!/usr/bin/env coffee

> @w5/redistream/init

for i from 'iaa clip civitai_img adult'.split ' '
  console.log i
  await init i
process.exit()
