# Clip Runtime

## [qdrant 的 docker](https://github.com/xxai-art/qdrant.docker)

```
git clone --depth=1 git@github.com:xxai-art/qdrant.docker.git
```

## 开发环境

请安装好以下软件（版本仅供参考，更高的版本应该也可以），可以用 [rtx](https://github.com/jdxcode/rtx) 管理版本。

* nodejs 20.2.0
* rust 1.72.0-nightly

[bun 0.6.7](https://bun.sh)

```
curl -fsSL https://bun.sh/install | bash
```

[direnv 2.32.2](https://direnv.net/docs/installation.html)

```
curl -sfL https://direnv.net/install.sh | bash
```

请配置好[direnv hook](https://direnv.net/docs/hook.html)。

然后 ./init.sh

`./init.sh` 会调用 [rust/ort.conf.coffee](./rust/ort.conf.coffee) 配置 ONNX 运行时支持的[加速器 (execution-providers)](https://github.com/pykeio/ort#execution-providers)，基于模板 [./rust/clip_runtime/Cargo.make.toml](./rust/clip_runtime/Cargo.make.toml) 生成 ONNX 加速器配置。

## qdrant

运行 ./qdrant.docker/up.sh 会启动一个 qdrant 。

./rust/clip_img2qdrant_grpc 可以通过 grpc 传入图片网址录入图片到向量数据库，用 bloomrpc 加载 ./rust/clip_img2qdrant_grpc/proto/img2qdrant.proto 可以调试。

## TODO

* rust 和 js 的接口文档

* 性能评测

## 相关链接

本项目参考了以下项目的代码 :

* [anansi](https://github.com/infrawhispers/anansi)
