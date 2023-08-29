#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR

exec watchexec --shell=none \
  --project-origin . -w ./src \
  --exts coffee,js,mjs,json,wasm,txt,yaml \
  -r \
  -- ./run.sh $exe
