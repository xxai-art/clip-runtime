#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

direnv allow

if [ ! -f "lib/model/$MODEL/process/tokenizer.json" ]; then
  mkdir -p lib/model
  cd lib/model
  wget -c $MODEL_URL
  FILE=$(basename $MODEL_URL)
  tar xvf $FILE
  rm $FILE
fi

cd $DIR
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

version=1.14.1

ORT_DLL=libonnxruntime.$version.$DLL_EXT

if [ ! -f "lib/so/$ORT_DLL" ]; then
  cd lib

  ort=onnxruntime-$OS-$(uname -m)

  if [ -x "$(command -v nvidia-smi)" ]; then
    ort=$ort-gpu
  fi

  ort=$ort-$version

  tgz=$ort.tgz
  wget -c https://github.com/microsoft/onnxruntime/releases/download/v$version/$tgz

  tar xvf $tgz
  rm -rf so
  mv $ort/lib so
  rm -rf $tgz $ort
fi

cd $DIR/lib/so
rm -rf onnxruntime.dll
ln -s $ORT_DLL onnxruntime.dll
cd $DIR

direnv exec . ./rust/ort.conf.coffee
