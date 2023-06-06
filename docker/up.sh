#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ ! -f "./.env" ]; then
  cp .env.example .env
fi

set -o allexport
source .env
set +o allexport

if [ ! -f "./qdrant/config.yml" ]; then
  cp qdrant/config.example.yml qdrant/config.yml
fi
