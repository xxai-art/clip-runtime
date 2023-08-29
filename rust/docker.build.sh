#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

name=art/img
docker build -t $name .
docker tag $name $name:latest
