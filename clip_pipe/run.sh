#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

./build.sh

if [ -f "/etc/profile" ]; then
  source /etc/profile
fi

nc -z -w 1 127.0.0.1 7890 && export https_proxy=http://127.0.0.1:7890

if [ ! -n "$1" ]; then
  exec ./lib/index.js
else
  exec ./${@:1}
fi
