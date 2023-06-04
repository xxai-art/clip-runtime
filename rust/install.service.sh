#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

name=$(grep "^name" Cargo.toml | sed 's/name = //g' | awk -F\" '{print $2}')

cp ./service /etc/systemd/system/$name.service

systemctl daemon-reload

systemctl enable --now $name
systemctl restart $name

systemctl status $name --no-pager

journalctl -u $name -n 10 --no-pager --no-hostname

mkdir -p /mnt/cache/nginx/$name-b2
mkdir -p /mnt/cache/nginx/$name-rust

for file in $DIR/nginx/*.conf; do
  filename=$(basename $file)
  if [ -L "/etc/nginx/site/$filename" ] || [ -f "/etc/nginx/site/$filename" ]; then
    rm "/etc/nginx/site/$filename"
  fi
  ln -s "$file" "/etc/nginx/site/$filename"
done

nginx -t && nginx -s reload
