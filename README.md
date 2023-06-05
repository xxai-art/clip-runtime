# Clip Runtime

## 开发环境

请安装好以下软件（版本仅供参考，更高的版本应该也可以），可以用 [rtx](https://github.com/jdxcode/rtx) 管理版本。

* nodejs 20.2.0
* rust 1.70.0

[bun 0.6.7](https://bun.sh)

```
curl -fsSL https://bun.sh/install | bash
```

[direnv 2.32.2](https://direnv.net/docs/installation.html)

```
curl -sfL https://direnv.net/install.sh | bash
```

请配置好[direnv hook](https://direnv.net/docs/hook.html)。

## Rust 编译说明

先运行 [rust/ort.conf.coffee](./rust/ort.conf.coffee) 配置 ONNX 运行时支持的[加速器 (execution-providers)](https://github.com/pykeio/ort#execution-providers)。

## 相关链接

本项目参考了以下项目的代码 :

* [anansi](https://github.com/infrawhispers/anansi)
