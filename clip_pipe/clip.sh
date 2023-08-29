#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ -f "/etc/profile" ]; then
  source /etc/profile
fi

exec ./lib/index.js
