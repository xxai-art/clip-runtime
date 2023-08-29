#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [[ "$OSTYPE" == "darwin"* ]]; then
  DLL_EXT=dylib
  OS=osx
  if ! [ -x "$(command -v yasm)" ]; then
    brew install yasm
  fi
elif [[ "$OSTYPE" == "linux"* ]]; then
  DLL_EXT=so
  OS=linux
  apt-get install -y libssl-dev pkg-config cmake
else
  DLL_EXT=dll
  OS=win
fi

version=$(curl -s https://api.github.com/repos/microsoft/onnxruntime/releases/latest | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')

ORT_DLL=libonnxruntime.$DLL_EXT

if [ ! -f "lib/so/$ORT_DLL" ]; then
  mkdir -p lib
  cd lib
  arch=$(uname -m)

  if [[ "$arch" == "x86_64" && "$OS" == "linux" ]]; then
    arch="x64"
  fi
  ort=onnxruntime-$OS-$arch

  if [ -x "$(command -v nvidia-smi)" ]; then
    ort=$ort-gpu
  fi

  ort=$ort-$version

  tgz=$ort.tgz
  wget --tries=999 -c ${GHPROXY}https://github.com/microsoft/onnxruntime/releases/download/v$version/$tgz

  tar xvf $tgz
  rm -rf so
  mv $ort/lib so
  rm -rf $tgz $ort
fi

cd $DIR/lib/so
rm -rf onnxruntime.dll
ln -s $ORT_DLL onnxruntime.dll
cd $DIR
