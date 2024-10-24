use std::process::exit;

use clap::Parser;
use pkg::package::Package;

use command::{run, Commands};
use config::Config;
use tokio::task;
use web;

#[derive(Parser, Debug)]
#[command(name = "rsup", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(flatten)]
    pkg_args: pkg::Args,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // 读取配置文件
    match Config::read_config().await {
        Ok(()) => {
            println!("读取配置文件成功!")
        }
        Err(e) => {
            eprintln!("读取配置文件失败: {}", e);

            exit(1)
        }
    };
    // 是否执行的自命令，则不需要启动pkg解析服务
    // match args.command {
    //     Commands::Config
    // }
    let package = Package::new();
    // 默认启动pkg解析服务

    let package_clone = package.clone();
    task::spawn(async move {
        pkg::run(args.pkg_args, package_clone).await;
    });

    web::run(package.clone()).await;
}
