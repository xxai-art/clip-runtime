#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

RUST_BACKTRACE=short cargo run -p ${1:-clip_cli}
