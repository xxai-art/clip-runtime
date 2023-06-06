#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ ! -f "./.env" ]; then
  cp .env.example .env
fi

set -o allexport
source .env
set +o allexport

CONFIG_FILE="qdrant/config.yml"

if [ ! -f "$CONFIG_FILE" ]; then
  cp qdrant/config.example.yml $CONFIG_FILE
fi

# 将所有以 QDRANT_ 开头的环境变量读入数组
vars=($(env | grep '^QDRANT_'))

# 遍历数组
for var in "${vars[@]}"; do
  # 移除 'QDRANT_' 前缀，获取 key 和 value
  key="${var#QDRANT_}"
  key="${key%%=*}"
  value="${var#*=}"

  # 尝试在配置文件中找到相应的 key
  if grep -q "^[[:space:]]*${key}:" "$CONFIG_FILE"; then
    # 找到了 key，替换它的值
    sed -i "s|^\([[:space:]]*${key}: \).*|\1${value}|" "$CONFIG_FILE"
  elif grep -q "^[[:space:]]*# ${key}" "$CONFIG_FILE"; then
    # 找到了被注释掉的 key，去掉注释并替换它的值
    sed -i "s|^\([[:space:]]*\)# ${key}.*|\1${key}: ${value}|" "$CONFIG_FILE"
  fi
done
