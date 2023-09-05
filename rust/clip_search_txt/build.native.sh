#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

export NATIVE=1

source ../sh/cflag.sh

cargo build $RUST_FEATURES --release --target $RUST_TARGET

name=$(grep "^name" Cargo.toml | sed 's/name = //g' | awk -F\" '{print $2}')

sudo mkdir -p /opt/bin
sudo mv target/$RUST_TARGET/release/server /opt/bin/$name

pre=/opt/bin/$name

if [ -f "$pre" ]; then
  rm -rf /tmp/$name
  sudo mv $pre /tmp
fi

case $(uname -s) in
Linux*)
  systemctl restart $name || ./service.sh
  sleep 5

  if ! systemctl is-active --quiet img.service; then
    journalctl -u $name -n 10 --no-pager --no-hostname
    echo -e "\n\n ❗服务启动失败\n\n"
    rm -rf /tmp/$name.failed
    mv /opt/bin/$name /tmp/$name.failed
    mv /tmp/$name /opt/bin/$name
    systemctl restart $name && sleep 5 || true
  fi

  systemctl status $name --no-pager
  journalctl -u $name -n 10 --no-pager --no-hostname
  ;;
esac
