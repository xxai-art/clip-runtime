#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

# if [ ! -d "jpegxl-rs" ]; then
#   ./sh/jpegxl-rs.sh
# fi

export RUST_LOG=$RUST_LOG,watchexec=off,watchexec_cli=off,globset=warn

exec watchexec \
  --shell=none \
  --project-origin . \
  -w . \
  --ignore "target/**" \
  --exts rs,sh,toml \
  -r \
  -- ./test.sh $@
