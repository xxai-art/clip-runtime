#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

docker run -v ./conf:/root/conf -v ./lib:/root/clip-runtime/lib -it --rm xxai/clip-cpu /bin/bash
