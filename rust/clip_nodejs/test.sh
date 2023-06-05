#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

rm -rf test/*.mjs
bunx cep -w -c coffee -o test -e mjs
./run.sh
