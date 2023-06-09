#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

bunx cep -c src -o lib
