#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! [ -x "$(command -v yasm)" ]; then
  if [[ "$(uname)" == "Darwin" ]]; then
    if ! command -v brew >/dev/null; then
      bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
    fi
    brew install yasm
  else
    sudo apt-get update
    sudo apt-get install -y yasm
  fi
fi

yarn
yarn build
