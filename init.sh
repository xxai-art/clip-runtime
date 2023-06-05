#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

rustc -vV | grep "host:"

mkdir -p lib/so
cd lib/so

version=1.14.1
name=onnxruntime-osx-arm64-$version

tgz=$name.tgz
wget -c https://github.com/microsoft/onnxruntime/releases/download/v$version/$tgz

rm -rf onnxruntime $name
tar zxvf $tgz
ln -s $name onnxruntime
rm -rf $tgz
cd onnxruntime/lib

ln -s libonnxruntime.dylib libonnxruntime.dll
