#!/usr/bin/env bash
DIR=$(realpath $0)
DIR=${DIR%/*}
cd $DIR
set -ex

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

source ./docker/root/.bashrc

curl --connect-timeout 2 -m 4 -s https://t.co >/dev/null || export GFW=1
[ $GFW ] &&
  git config --global url."https://ghproxy.com/https://github.com".insteadOf "https://github.com" &&
  pip config set global.index-url https://mirrors.aliyun.com/pypi/simple/,https://pypi.doubanio.com/simple/ &&
  export npmmirror=https://registry.npmmirror.com &&
  export GHPROXY=https://ghproxy.com/ &&
  export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static &&
  export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

apt-get update
apt-get install -y \
  wget yasm tar unzip zstd git direnv tmux gcc git-lfs bzip2 htop g++ bash libssl-dev pkg-config cmake pbzip2 curl rsync netcat-openbsd psmisc
# killall -> psmisc

direnv allow
direnv exec . ./down.model.sh

if [ ! -f "$HOME/.cargo/env" ]; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain nightly
fi

source "$HOME/.cargo/env"

to_install=""

# 检查rtx是否存在
if ! command -v rtx &>/dev/null; then
  to_install+="rtx-cli "
fi

# 检查fd是否存在
if ! command -v fd &>/dev/null; then
  to_install+="fd-find "
fi

if [ -n "$GFW" ]; then
  nc -z -w 1 127.0.0.1 7890 && export HTTPS_PROXY=http://127.0.0.1:7890
fi

if [ -n "$to_install" ]; then
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash &&
    cargo binstall --no-confirm --no-symlinks $to_install || cargo install $to_install
fi

source ./docker/root/.bashrc

NODE_VER=20.5.1
rtx install node@$NODE_VER
rtx global node@$NODE_VER

source ./docker/root/.bashrc

if [ -n "$GFW" ]; then
  npm config set registry $npmmirror
fi

ensure() {
  for pkg in "$@"; do
    if ! command -v $pkg &>/dev/null; then
      npm install -g $pkg
    fi
  done
}

ensure pnpm yarn

if [ -n "$GFW" ]; then
  yarn config set registry $npmmirror
fi

source ./env

if ! command -v bun &>/dev/null; then
  curl -fsSL https://bun.sh/install | bash
fi

./down.dll.sh
direnv exec . ./init.sh
# git clone --depth=1 https://github.com/xxai-art/clip-runtime.git

#   source ~/.bashrc

# cd clip-runtime

cd rust/clip_img2qdrant

./dist.sh

cd $DIR/clip_pipe
pnpm i

if [ -n "$GFW" ]; then
  direnv exec . ./supervisor.sh
  tail -f /var/log/supervisor/xxai-*.log
else
  direnv exec . ./run.sh
fi
