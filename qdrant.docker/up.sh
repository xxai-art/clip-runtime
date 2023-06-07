#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

direnv allow

if [ ! -f "./.env" ]; then
  cp .env.example .env
  KEY=$(openssl rand -base64 15 | tr '/+' '_-')
  echo -e "\nQDRANT__SERVICE__API_KEY=$KEY\n" >>.env
fi

set -o allexport
source .env
set +o allexport

docker-compose down || true
docker-compose up -d
sleep 1
direnv exec . ./init.coffee
