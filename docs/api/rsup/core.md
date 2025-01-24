
# `core` {#core}

`rsup` 核心入口模块。通过解析命令行参数，调用不同的功能模块。

主入口`main.rs`函数定义如下：

```rust:line-numbers {1}
use clap::Parser;
use pkg::package::Package;

use command::{run, Commands};
use tokio::task;
use web;

#[derive(Parser, Debug)]
#[command(name = "rsup", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    pkg_args: pkg::Args,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Config { .. }) | Some(Commands::Update { .. }) => {
            run().await;
        }
        _ => {
            let package = Package::new();
            // 默认启动pkg解析服务

            let package_clone = package.clone();
            task::spawn(async move {
                pkg::run(args.pkg_args, package_clone).await;
            });

            web::run(package.clone()).await;
        }
    }
}

```

根据命令解析调用不同的服务，目前包含两种服务`command` 和`web`服务。

* `command`服务用于解析命令行参数，在命令行中交互操作功能。[查看文档](./command)
* `web`服务用于启动web服务，提供web界面交互操作功能。[查看文档](./web)

    `web`服务通常是和`pkg`功能包一起使用的。`pkg`用于解析前端项目依赖文件，`web`包则提供与前端页面交互的能力。

    > [!NOTE]
    > `rsup` 命令行工具默认启动`web`服务并调用`pkg`功能包。
