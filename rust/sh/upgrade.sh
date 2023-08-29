#!/usr/bin/env bash
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -ex
cargo update
cargo upgrade
cd ../clip_nodejs/lib/napi-rs
git checkout .
