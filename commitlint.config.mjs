export default {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [
      2,
      "always",
      [
        "feat", // 新功能
        "fix", // 修复bug
        "docs", // 文档修改
        "style", // 样式修改，代码格式
        "refactor", // 重构代码
        "perf", // 性能优化
        "test", // 测试相关
        "build", // 构建相关
        "ci", // 持续集成
        "chore", // 构建流程或辅助工具的变动
      ],
    ],
  },
};
