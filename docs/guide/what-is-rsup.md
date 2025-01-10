---
layout: doc
---

# 什么是Rsup? {#what-is-rsup}

`rsup`是一个前端项目依赖管理助手，它可以帮助你实现web可视化添加、删除、升级依赖。并且提供了某个依赖的关系图，方便查看依赖的底层依赖。

<div class="tip custom-block" style="padding-top: 8px">

尝试一下？跳到[开始安装](./intaller/macos)。

</div>

## 使用场景 {#use-scase}

所有前端项目中有`npm`依赖的项目都可以使用`rsup`去管理依赖。支持`npm` \ `pnpm` \ `yarn`。可以批量安装、批量卸载依赖，利用`rust`的异步编程能力，可同时执行多个操作。
