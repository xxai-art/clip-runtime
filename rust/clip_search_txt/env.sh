#!/usr/bin/env bash
DIR=$(dirname "${BASH_SOURCE[0]}")
cd $DIR
set -ex
export RUST_LOG="debug,globset=warn,watchexec=off,supervisor=warn,hyper=warn,rustls=warn,h2=warn,tower=warn,ort=warn"

ROOT=$(realpath $DIR/../..)
export ORT_LIB_LOCATION=$ROOT/lib/so
export ORT_DYLIB_PATH=$ORT_LIB_LOCATION/onnxruntime.dll
export MODEL_DIR=$ROOT/lib/model/AltCLIP-XLMR-L-m18

env_sh() {
  local dir_conn=$(dirname $ROOT)/conf/conn
  if [ ! -d "$dir_conn" ]; then
    echo "$dir_conn not exist"
    return
  fi

  cd $dir_conn
  local i
  for i in $@; do
    if [ -f "$i.sh" ]; then
      set -o allexport
      source "$i".sh
      set +o allexport
    else
      echo "$i not exist"
    fi
  done

  cd $DIR
  unset -f env_sh
}

env_sh host qdrant
