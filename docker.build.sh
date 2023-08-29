#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

img=xxai/clip-cpu
tag=$(date +%Y-%m-%d)

cp tmpl.dockerfile Dockerfile
sed -i 's/NODE_VER/20.5.1/g' Dockerfile
docker build -t $img:$tag -t $img:latest .
