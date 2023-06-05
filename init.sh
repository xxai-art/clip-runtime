#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

direnv allow

rustc -vV | grep "host:"

version=1.14.1

if [ ! -f "lib/so/onnxruntime.$version.dll" ]; then
  if [[ "$OSTYPE" == "darwin"* ]]; then
    DLL_EXT=dylib
    OS=osx
  elif [[ "$OSTYPE" == "linux"* ]]; then
    DLL_EXT=so
    OS=linux
  else
    DLL_EXT=dll
    OS=win
  fi

  ort=onnxruntime-$OS-$(uname -m)

  if [ -x "$(command -v nvidia-smi)" ]; then
    ort=$ort-gpu
  fi

  ort=$ort-$version
  mkdir -p lib/so
  cd lib/so

  tgz=$ort.tgz
  wget -c https://github.com/microsoft/onnxruntime/releases/download/v$version/$tgz

  tar zxvf $tgz
  mv $ort/lib/*.$DLL_EXT $DIR/lib/so/
  rm -rf $tgz $ort
fi

cd $DIR/lib
rm -rf onnxruntime.dll
ln -s lib/onnxruntime.$version.dll lib/onnxruntime.dll
cd $DIR

direnv exec . ./rust/ort.conf.coffee
