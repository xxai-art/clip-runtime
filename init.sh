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
  elif [[ "$OSTYPE" == "linux"* ]] || [[ "$OSTYPE" == *"bsd"* ]]; then
    DLL_EXT=so
  else
    DLL_EXT=dll
  fi
  ort=onnxruntime-osx-arm64-$version
  mkdir -p lib/so
  cd lib/so

  tgz=$ort.tgz
  wget -c https://github.com/microsoft/onnxruntime/releases/download/v$version/$tgz

  tar zxvf $tgz
  mv $ort/lib/libonnxruntime.$version.$DLL_EXT $DIR/lib/so/onnxruntime.$version.dll
  rm -rf $tgz $ort
fi

cd $DIR/lib
rm -rf onnxruntime.dll
ln -s lib/onnxruntime.$version.dll lib/onnxruntime.dll
cd $DIR
