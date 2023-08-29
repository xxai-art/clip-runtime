#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ ! -f "lib/model/$MODEL/process/tokenizer.json" ]; then
  mkdir -p lib/model
  cd lib/model

  wget --tries=999 -c $MODEL_URL
  FILE=$(basename $MODEL_URL)
  if [ -x "$(command -v apt-get)" ]; then
    if ! [ -x "$(command -v zstd)" ]; then
      apt-get install -y zstd
    fi
    if ! [ -x "$(command -v pv)" ]; then
      apt-get install -y pv
    fi
    pv $FILE | zstd -d -c -T0 | tar xf -
  else
    zstd -d -c -T0 $FILE | tar xf -
  fi
  rm $FILE
fi
