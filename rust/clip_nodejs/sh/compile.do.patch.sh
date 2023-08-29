#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

bunx cep -c do.patch.coffee

sed -i '1s|^.*$|#!/usr/bin/env node|' do.patch.js
