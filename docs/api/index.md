---
layout: doc
---

# API {#api}

`rsup`设计功能拆分为不同的功能子包，利用`cargo`的空间管理功能，协同各个包之间可以相互依赖。

## `core` {#api-core}

`core`包是`rsup`的核心包，它是程序的主入口，负责解析命令行参数，调用其他子包的功能。[查看core包文档](./core)

## `config` {#api-config}

`config`包是`rsup`的配置文件解析包，负责解析配置文件，并生成配置对象。[查看config包文档](./config)

## `command` {#api-command}

`command`包是`rsup`的子命令命令，执行命令行命令。它可以在命令行中执行操作一系列功能。[查看command包文档](./command)

## `pkg` {#api-pkg}

`pkg`包是`rsup`解析前端项目依赖的核心包，包括解析`package.json` \ `package-lock.json`等文件，包括`pnpm`和`yarn`管理的项目。[查看pkg包文档](./pkg)

## `web` {#api-web}

`web`包是`rsup`的web服务包，负责启动web服务，并接收并处理前端请求。[查看web包文档](./web)

## `utils` {#api-utils}

`utils`包是`rsup`的工具包，负责提供一些通用的工具函数，包括文件下载、解压、环境变量解析等。[查看utils包文档](./utils)
