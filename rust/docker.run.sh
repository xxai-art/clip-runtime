#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

IMG=art/img

NAME=$(basename $IMG)

PORT=${1:-"9911"}

(docker stop $NAME || true) && exec docker run --rm -e PORT=$PORT -p$PORT:$PORT --name $NAME $IMG
