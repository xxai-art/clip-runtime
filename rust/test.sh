#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cargo test -p ${1:-clip_runtime} -- --nocapture
