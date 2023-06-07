#!/usr/bin/env coffee

> @w5/avat

I = await import('../index.js')
T = avat I

T.helloWorld([1,2,3])(3)
