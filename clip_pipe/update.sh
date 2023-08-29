#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

git pull -f
bunx cep -c src -o lib
cd ../rust/clip_img2qdrant
./dist.sh
killall -9 node
