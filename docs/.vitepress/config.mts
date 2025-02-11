import { defineConfig } from "vitepress";
import { createRequire } from "module";
const require = createRequire(import.meta.url);
const pkg = require("../../package.json");

// https://vitepress.dev/reference/site-config
export default defineConfig({
  lang: "zh-CN",
  title: "Rsup",
  titleTemplate: ":title | Npm Helper",
  description: "A Simple helper for npm pakcage",
  themeConfig: {
    logo: "/logo.png",
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "指南", link: "/guide/what-is-rsup" },
      { text: "API", link: "/api" },
      { text: "演示", link: "/example" },
      {
        text: pkg.version,
        items: [
          {
            text: "更新日志",
            link: "https://github.com/ngd-b/rsup/releases",
          },
        ],
      },
    ],

    sidebar: {
      "/guide": {
        base: "/guide/",
        items: [
          {
            text: "简介",
            collapsed: false,
            items: [
              { text: "什么是rsup？", link: "what-is-rsup" },
              {
                text: "安装",
                base: "/guide/installer/",
                items: [
                  { text: "macos", link: "macos" },
                  { text: "windows", link: "windows" },
                ],
              },
            ],
          },
          {
            text: "开始使用",
            collapsed: false,
            base: "/guide/start/",
            items: [
              {
                text: "rsup使用",
                link: "base",
                items: [{ text: "命令行功能", link: "command" }],
              },
              { text: "web功能", link: "web" },
              { text: "配置文件", link: "config" },
            ],
          },
        ],
      },
      "/api": {
        base: "/api/",
        items: [
          {
            text: "Api",
            link: "index",
            items: [
              {
                text: "rsup",
                base: "/api/rsup/",
                collapsed: false,
                items: [
                  { text: "core", link: "core" },
                  { text: "config", link: "config" },
                  { text: "command", link: "command" },
                  { text: "pkg", link: "pkg" },
                  { text: "web", link: "web" },
                  { text: "utils", link: "utils" },
                ],
              },
              {
                text: "rsup-installer",
                base: "/api/rsup-installer/",
                collapsed: false,
                items: [{ text: "core", link: "core" }],
              },
            ],
          },
        ],
      },
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/ngd-b/rsup" },
      { icon: "juejin", link: "https://juejin.cn/user/2084329777543191/posts" },
      { icon: "csdn", link: "https://blog.csdn.net/heroboyluck" },
    ],
    footer: {
      message: "Released under the Apache License.",
      copyright: "Copyright © 2024-present hboot",
    },
    lastUpdated: {
      text: "最后更新于",
      formatOptions: {
        dateStyle: "short",
        timeStyle: "medium",
      },
    },
    docFooter: {
      prev: "上一页",
      next: "下一页",
    },
    editLink: {
      pattern: "https://github.com/ngd-b/rsup/edit/main/docs/:path",
      text: "在 GitHub 上编辑此页面",
    },
    outline: {
      level: [2, 6],
      label: "页面导航",
    },
    search: {
      provider: "local",
    },
  },
  // base: "/rsup/",
  cleanUrls: true,
  srcExclude: ["**/node_modules/**", "**/README.md", "**/TODO.md"],
});
