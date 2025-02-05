
# `rsup` 配置文件

配置文件包括了以下内容：

```toml
name = "rsup"
version = "0.3.0"
dir = "/opt/rsup"

[web]
port = 8888
static_dir = "/opt/rsup/web"

[pkg]
npm_registry = "https://registry.npmmirror.com"
```

最主要的信息`web.static_dir`指定了静态资源的存放地址。`pkg.npm_registry`指定了`npm`仓库地址。

通常不需要去修改配置文件，在[安装时](../installer/macos)时，默认生成到指定路径下。你可以在足够了解`rsup`的设计结构的情况下去修改配置文件。
