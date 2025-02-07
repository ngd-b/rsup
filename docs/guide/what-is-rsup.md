
# 什么是Rsup? {#what-is-rsup}

`rsup`是一个前端项目依赖管理助手，它可以帮助你实现web可视化添加、删除、升级项目的依赖。并且可以查询某个依赖的关系图，方便查看依赖的底层依赖。

<div class="tip custom-block" style="padding-top: 8px">

尝试一下？跳到[开始安装](./installer/macos)。

</div>

## 使用场景 {#use-scase}

* 日常项目依赖管理 <Badge type="info" text="持续完善" />

  所有前端项目中有`npm`依赖的项目都可以使用`rsup`去管理依赖。支持`npm` \ `pnpm` \ `yarn`。可以批量安装、批量卸载依赖，利用`rust`的异步编程能力，可同时执行多个操作。

* 查看依赖关系图 <Badge type="tip" text="+npm" /><Badge type="tip" text="+pnpm" /><Badge type="tip" text="+yarn" />

  提供了查看某个依赖的依赖关系图，方便查看依赖的底层依赖。从而分析依赖之间的相互关系，解决依赖之间冲突问题。

  > [!NOTE]
  > 依赖关系解析的是管理器安装依赖后生成的`lock`文件，`npm` \ `pnpm` \ `yarn`安装的逻辑各不相同，他们之间不同的版本安装的逻辑也各不相同。

* 旧项目升级 <Badge type="warning" text="待实现" />

  根据项目配置，提供整个项目升级模板，通过web可视化的方式升级项目。
