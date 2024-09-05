<h1 style="text-align:center;">rsup</h1>

<p style="text-align:center;">A simple helper for npm package</p>

<p style="text-align:center;">
    [![star](https://gitee.com/hboot/rsup/badge/star.svg?theme=white)](https://gitee.com/hboot/rsup/stargazers)

</p>

## 介绍

rust 工具库用来升级前端项目。旧项目迁移新脚手架工具。

- 👀 通过 web 服务查看 npm 项目依赖包新版本信息
- 🚀 一键升级项目依赖包版本

## 安装

1. 直接下载并安装

[github 下载地址](https://github.com/ngd-b/rsup/releases/download/latest/rsup.tar.gz)

[gitee 下载地址](https://gitee.com/hboot/rsup/releases/download/latest/rsup.tar.gz)

解压文件,使用解压工具解压，或者使用命令行工具解压，得到一个可以执行文件。

```sh
$> tar -xzvf rsup.tar.gz
```

在终端将执行文件移动到`/usr/local/bin`目录下，使得`rsup`命令全局可用。或者直接在解压后的文件夹中执行。

```sh
$> sudo mv rsup /usr/local/bin/
```

2. 使用安装脚本安装

脚本地址`https://gitee.com/hboot/rsup/raw/master/install.sh`

在终端执行命令

```sh
# github
$> curl -fsSL https://github.com/ngd-b/rsup/raw/main/install.sh | bash

# gitee
$> curl -fsSL https://gitee.com/hboot/rsup/raw/master/install.sh | bash
```

提示安装成功后，就可以在终端执行`rsup`命令。
