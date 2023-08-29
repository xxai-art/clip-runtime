#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

direnv allow
git submodule update --init --recursive

./rust/sh/provider.coffee
#./rust/sh/jpegxl-rs.sh
