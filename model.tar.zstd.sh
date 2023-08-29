#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd ./lib/model
model=AltCLIP-XLMR-L-m18
tar cf - $model | zstd -T0 -15 -o $model.tar.zstd
