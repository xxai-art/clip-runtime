#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
cd ..
napi build --platform $@
$DIR/do.patch.js
