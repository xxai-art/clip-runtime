#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

exec watchexec \
  --shell=none \
  --project-origin . -w ./src \
  --exts rs,toml \
  -r \
  -- ./run.sh
