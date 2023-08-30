#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

./build.sh

nc -z -w 1 127.0.0.1 7890 && export http_proxy=http://127.0.0.1:7890 https_proxy=$http_proxy

if [ ! -n "$1" ]; then
  exec ./lib/index.js
else
  exec ./${@:1}
fi
