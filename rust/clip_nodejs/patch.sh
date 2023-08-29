#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e

INDEX=$(cat ./index.js)
PATCH=$(cat ./patch.js)
echo -e "$INDEX;\n$PATCH" >index.js
