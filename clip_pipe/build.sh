#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

bun x cep -c src -o lib
