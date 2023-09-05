#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

name=$(grep "^name" Cargo.toml | sed 's/name = //g' | awk -F\" '{print $2}')

EXE=/opt/bin/$name

if [ ! -f "$EXE" ]; then
  ./dist.native.sh
fi

system_service=/etc/systemd/system/$name.service
cp ./service $system_service

if [ -n "$TO" ]; then
  sed -i 's#Environment="TO=.*"#Environment="TO='"$TO"'"#' $system_service
fi

systemctl daemon-reload

systemctl enable --now $name
systemctl restart $name

systemctl status $name --no-pager

journalctl -u $name -n 10 --no-pager --no-hostname
