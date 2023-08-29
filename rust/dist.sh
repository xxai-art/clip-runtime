#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# ./sh/jpegxl-rs.sh
source ./sh/cflag.sh

project=clip_img2qdrant_grpc

cargo build $RUST_FEATURES --release --target $RUST_TARGET -p $project
cp target/$RUST_TARGET/release/$project target
