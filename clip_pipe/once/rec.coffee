#!/usr/bin/env coffee

> path > join
  @w5/uridir
  @w5/read

ROOT = uridir(import.meta)
for i from read join ROOT,'./rec.init.txt'
  console.log(i)



