
# `command` {#command}

`command`作为`rsup`的子命令行工具，提供了在命令中交互的能力。关于命令行的使用，[查看命令行功能](/guide/start/command)

提供了`Config`和`Update`两个子命令。

```rs:line-numbers
#[derive(Parser, Debug)]
pub enum Commands {
    #[clap(name = "config", about = "Manage the config file")]
    Config {
        #[clap(subcommand)]
        config: ConfigOptions,
    },
    #[clap(name = "update", about = "Update the rsup binary and web client")]
    Update {
        #[clap(subcommand)]
        update: UpdateOptions,
    },
}
```

## `Config` {#command-config}

用于管理`rsup`的配置文件`config.toml`,提供诸如查看、获取、设置的能力。

```rs:line-numbers
#[derive(Parser, Debug)]
pub enum Options {
    #[clap(name = "list", about = "List all config attributes")]
    List,
    #[clap(name = "set", about = "Set config value")]
    Set { key: String, value: String },
    #[clap(name = "get", about = "Get config value")]
    Get { key: String },
    #[clap(name = "delete", about = "Delete config value")]
    Delete,
}
```

## `Update` {#command-update}

用于更新`rsup`自身。包括了`rsup`可执行文件、`web`静态服务资源

```rs:line-numbers
#[derive(Parser, Debug)]
pub enum Options {
    Rsup,
    Web,
}
```
