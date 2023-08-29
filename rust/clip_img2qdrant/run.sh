#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

yarn run build:debug

catout() {
  glow -s dracula out.txt
}
RUST_BACKTRACE=short yarn test 2>out.txt || (catout && exit 1)

catout
