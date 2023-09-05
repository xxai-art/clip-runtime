#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
export RUST_LOG="debug,globset=warn,watchexec=off,supervisor=warn,hyper=warn,rustls=warn,h2=warn,tower=warn,ort=warn"

ROOT=$(realpath $DIR/../..)
export ORT_LIB_LOCATION=$ROOT/lib/so
export ORT_DYLIB_PATH=$ORT_LIB_LOCATION/onnxruntime.dll
export MODEL_DIR=$ROOT/lib/model/AltCLIP-XLMR-L-m18
