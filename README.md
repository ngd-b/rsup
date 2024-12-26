<h1 style="text-align:center;">rsup</h1>

<p style="text-align:center;">A simple helper for npm package</p>

<p style="text-align:center;">
  
</p>

## 介绍

rust 工具库用来升级前端项目。旧项目迁移新脚手架工具。

- 通过 web 服务查看 npm 项目的依赖
- 查看某个依赖所有的新版本信息
- 一键升级项目依赖包版本
- 查看 npm 包依赖树

## 安装

`rsup`命令包含了配置文件、可执行文件、web服务文件等。根据不同的系统，提供了三种工具安装包包括linux、macos、windows。

[macos installer](https://github.com/ngd-b/rsup/releases/download/latest/rsup-installer-macos-latest.tar.gz)

[ubuntu instanller](https://github.com/ngd-b/rsup/releases/download/latest/rsup-installer-ubuntu-latest.tar.gz)

[windows instanller](https://github.com/ngd-b/rsup/releases/download/latest/rsup-installer-windows-latest.zip)

1. **推荐** 使用脚本安装

提供了安装脚本文件`sh`一键下载解压、安装。无需手动配置环境变量。

```sh
curl -fsSL https://raw.githubusercontent.com/ngd-b/rsup/main/install.sh | sh
```

根据提示选择，安装完成即可。

2. **手动安装** 下载工具包

`windows` 用户需要手动下载安装包，解压执行`installer`即可。其他系统也可以手动下载安装包。

手动下载安装包。执行脚本解压、执行脚本，需要管理员权限`sudo`

解压文件,使用解压工具解压，或者使用命令行工具解压，得到一个`installer`可以执行文件。

```sh
tar -xzvf rsup-installer-macos-latest.tar.gz
```

在终端将执行执行文件`installer`。

```sh
sudo ./installer
```

根据提示安装成功后，就可以在终端执行`rsup`命令。默认安装目录为`/opt/rsup`
