#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# ./sh/jpegxl-rs.sh
source ./sh/cflag.sh

cargo build $RUST_FEATURES --release --target $RUST_TARGET
