use std::sync::Arc;

use clap::Parser;
use pkg;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tokio::task;
use web;
mod package;
#[derive(Parser, Debug)]
#[command(name = "rsup", author, version, about)]
struct Cli {
    // #[command(subcommand)]
    // command: Commands,
    #[clap(flatten)]
    pkg_args: pkg::Args,
}

// #[derive(Subcommand, Debug)]
// enum Commands {
//     Pkg(pkg::Args),
// }

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let data = Arc::new(Mutex::new(pkg::Pkg::new()));

    let (tx, rx) = channel(100);

    // match args.command {
    //     Commands::Pkg(args) => {
    //         let data_clone = data.clone();
    //         let tx_clone = tx.clone();
    //         task::spawn(async move {
    //             if let Err(e) = pkg::run(args, data_clone, tx_clone).await {
    //                 println!("Error parse package.json  {}", e);
    //             };
    //         });
    //     }
    // }

    // 默认启动pkg解析服务

    let data_clone = data.clone();
    let tx_clone = tx.clone();
    task::spawn(async move {
        if let Err(e) = pkg::run(args.pkg_args, data_clone, tx_clone).await {
            println!("Error parse package.json  {}", e);
        };
    });

    let _ = web::run(Arc::clone(&data.clone()), rx).await;
}
