---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Rsup"
  text: "前端项目依赖管理助手"
  actions:
    - theme: brand
      text: 什么是rsup？
      link: /guide/what-is-rsup
    - theme: alt
      text: 快速开始
      link: /guide/installer/macos
    - theme: alt
      text: 查看演示
      link: /example

features:
  - icon: ⛏️
    title: 管理依赖
    details: 解析 package.json 文件，获取依赖最新的版本信息；可进行依赖的安装、更新、删除等操作；展示依赖关系图
  - icon: ⚡
    title: 执行迅速
    details: 使用rust编写，执行速度快，内存安全。
  - icon: 📦
  #  title: 兼容性
  # details: 支持npm、pnpm；安装包提供windows、mac、linux版本。
---

