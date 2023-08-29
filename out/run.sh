#!/usr/bin/env bash

OUT=$(realpath $0) && OUT=${OUT%/*}
DIR=$(dirname $OUT)
set -ex
source $DIR/env
exec node $OUT/index.js
