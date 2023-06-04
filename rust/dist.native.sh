#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

export NATIVE=1

# ./sh/jpegxl-rs.sh

source ./sh/cflag.sh

cargo build $RUST_FEATURES --release --target $RUST_TARGET

if [[ $(uname -s) == Linux ]]; then
  mkdir -p /mnt/bin

  name=$(grep "^name" Cargo.toml | sed 's/name = //g' | awk -F\" '{print $2}')

  pre=/mnt/bin/$name

  if [ -f "$pre" ]; then
    rm -rf /tmp/$name
    mv $pre /tmp
  fi

  mv target/$RUST_TARGET/release/$name /mnt/bin
  systemctl restart $name
  sleep 5

  if ! systemctl is-active --quiet img.service; then
    journalctl -u $name -n 10 --no-pager --no-hostname
    echo -e "\n\n ❗服务启动失败\n\n"
    rm -rf /tmp/$name.failed
    mv /mnt/bin/$name /tmp/$name.failed
    mv /tmp/$name /mnt/bin/$name
    systemctl restart $name && sleep 5 || true
  fi

  systemctl status $name --no-pager
  journalctl -u $name -n 10 --no-pager --no-hostname
fi
