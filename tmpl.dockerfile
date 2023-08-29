ARG UBUNTU_VER=23.10 

FROM ubuntu:$UBUNTU_VER as build

SHELL ["/bin/bash", "-c"]

ENV DEBIAN_FRONTEND noninteractive
ENV TERM xterm-256color
RUN apt-get update && apt-get install -y \
wget yasm tar unzip zstd git direnv tmux gcc git-lfs bzip2 htop g++ bash libssl-dev pkg-config cmake pbzip2 curl && \
curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain nightly &&\
source "$HOME/.cargo/env" &&\
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash &&\
cargo binstall --no-confirm --no-symlinks rtx-cli fd-find

COPY ./docker/ /

RUN source ~/.bashrc &&\ 
rtx install node@NODE_VER &&\
rtx global node@NODE_VER

RUN source ~/.bashrc &&\ 
npm install -g pnpm yarn &&\
curl -fsSL https://bun.sh/install | bash

WORKDIR /root
RUN \
git clone --depth=1 https://github.com/xxai-art/clip-runtime.git &&\
source ~/.bashrc &&\ 
cd clip-runtime &&  direnv allow && direnv exec . ./init.sh &&\
cd rust/clip_img2qdrant &&\
./dist.sh

RUN source ~/.bashrc &&\ 
cd clip-runtime/clip_pipe && pnpm i &&\
./build.sh && cd ../export && bun i && direnv exec . ./export.sh

FROM ubuntu:$UBUNTU_VER
ENV RUST_BACKTRACE=short
# COPY --from=build /so /so
COPY --from=build /root/clip-runtime/out /root/clip-runtime/out
COPY --from=build /root/clip-runtime/env /root/clip-runtime/env
COPY --from=build /root/.local/share/rtx/installs/node/ /root/.local/share/rtx/installs/node/
ENV PATH /root/.local/share/rtx/installs/node/NODE_VER/bin:/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin:/sbin:/bin
CMD ["/root/clip-runtime/out/run.sh"]


