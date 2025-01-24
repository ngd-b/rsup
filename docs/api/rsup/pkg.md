
# `Pkg` {#pkg}

`pkg` 包用于解析前端依赖文件，包括`package.json` \ `package-lock.json` 。对于不同依赖管理器，则对应不同的配置文件。比如`pnpm-lock.yaml` \ `yarn.lock`.

在[开始](/guide/installer/macos)中创建了`rsup`的配置文件`config.toml`,其中包含了`pkg`服务的配置。

```toml
[pkg]
npm_registry = "https://registry.npmmirror.com"
```

设置依赖源地址，默认是`npmmirror`源，可以设置为其他源地址。可以查看[如何修改配置](/guide/start/command)
