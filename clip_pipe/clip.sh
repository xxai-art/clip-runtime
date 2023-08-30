#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ -f "/etc/profile" ]; then
  source /etc/profile
fi

nc -z -w 1 127.0.0.1 7890 && export http_proxy=http://127.0.0.1:7890 https_proxy=$http_proxy
exec ./lib/index.js
