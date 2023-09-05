#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

direnv allow
git submodule update --init --recursive

./rust/sh/provider.coffee
./down.dll.sh
./down.model.sh
#./rust/sh/jpegxl-rs.sh
