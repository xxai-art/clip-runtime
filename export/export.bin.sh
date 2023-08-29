#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./export.sh
if [ -d "clip-bin" ]; then
  cd clip-bin
  git pull
else
  git clone --depth=1 git@github.com:xxai-art/clip-bin.git
fi

cd $(dirname $DIR)/out
if [ -x "$(command -v nvidia-smi)" ]; then
  kind=gpu
else
  kind=cpu
fi

to=$DIR/clip-bin/$kind.tar.zstd
rm -rf $to
tar cf - * | zstd -T0 -10 -o $to

cd $DIR/clip-bin
git add .
git commit -m.
git push
