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
