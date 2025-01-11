# 命令行功能 {#command}

`rsup`除了提供`web`交互功能外，还支持命令行交互功能。方便在没有浏览器的情况下使用。

> [!info]
> 目前还未提供管理项目依赖的命令。

## `config` {#command-config}

`config`命令用于配置管理`rsup`的配置文件`config.toml`。在[安装](../installer/macos)一节中介绍了`rsup`的配置文件`config.toml`,包括配置文件的内容和配置文件的位置。

具体内容如下，这是`macos`下的，其他系统内容类似：

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

这些内容你可以直接通过`vi`修改，也可以通过提供的`config`命令进行修改。

### `list` {#command-config-list}

展示出所有配置信息。

```sh
rsup config list
```

### `set` {#command-config-set}

设置配置信息。

```sh
rsup config set web.port 8889
```

### `get` {#command-config-get}

获取配置信息。

```sh
rsup config get web.port
```

## `update` {#command-update}

`update`命令用于更新`rsup`和`web`服务。提供了方便更新服务的功能，这对于更新`rsup`和`web`服务来说十分有用。当然也可以手动下载后覆盖旧版本。

更新`rsup`

```sh
rsup update rsup
```

更新`web`服务

```sh
rsup update web
```

## `uninstall` {#command-uninstall}

`uninstall`命令用于卸载`rsup`工具。

卸载包括删除`rsup`的所有文件以及目录`/opt/rsup`；并且会清除`rsup`的环境变量。
