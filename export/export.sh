#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ ! -d "node_modules" ]; then
  ni
fi

./esbuild.coffee

ROOT=$(dirname $DIR)

OUT=$ROOT/out

PROJECT_DIR=$ROOT/clip_pipe

cd $PROJECT_DIR

cd node_modules

fd ".*\.wasm$" --no-ignore --hidden |
  xargs -I {} cp {} $OUT

IMG2QD=$ROOT/rust/clip_img2qdrant

NODE_FILE=$(ls $IMG2QD/*.node)

cp $NODE_FILE $OUT

platform=$(basename $NODE_FILE | awk -F '.' '{print $(NF-1)}')

cd $PROJECT_DIR/node_modules/.pnpm/@w5+xedis-$platform@*/node_modules/@w5/xedis-$platform/

cp *.node $OUT

# cd $OUT
# node --experimental-sea-config exe.json
# cp $(command -v node) main
# npx postject main NODE_SEA_BLOB exe.blob \
#   --sentinel-fuse NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2

# cpso() {
#   mkdir -p /so
#   ldd $1 | grep "=> /" | awk '{print $3}' | xargs -I '{}' sh -c 'cp -L "{}" /so/'
# }
#
# cpso $OUT/$(basename $NODE_FILE)
# cpso $OUT/main
# cpso $(which node)
# cd $ROOT/..

# name=clip-runtime.tar.zstd
# tar cf - clip-runtime/.envrc clip-runtime/out clip-runtime/lib conf/conn | zstd -15 -o $name

# rclone copy $name g:/art/
# rm -rf $name
