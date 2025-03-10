use clap::{value_parser, Parser};
use pkg::package::Package;

use command::{run, Commands};
use tokio::task;
use web;

#[derive(Parser, Debug)]
#[command(name = "rsup", author, about, disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    pkg_args: pkg::Args,
    #[arg(
        short,
        long,
        default_value = "false",
        value_parser=value_parser!(bool),
        help = "Default open browser when servier start"
    )]
    quit: bool,
    #[arg(short, long, help = "Show version about rsup and rsup-web")]
    version: bool,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if args.version {
        command::show_version().await;
        return;
    }
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

            web::run(package.clone(), !args.quit).await;
        }
    }
}
