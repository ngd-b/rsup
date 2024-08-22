use std::sync::Arc;

use clap::{Parser, Subcommand};
use pkg;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tokio::task;
use web;
mod package;
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Pkg(pkg::Args),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let data = Arc::new(Mutex::new(pkg::Pkg::new()));

    // let (tx, rx) = broadcast::channel(10);
    // let package = Pakcage::new();
    let (tx, rx) = channel(1);

    match args.command {
        Commands::Pkg(args) => {
            // let data_clone = Arc::clone(&package.pkg);

            // let tx_clone = package.sender.clone();

            let data_clone = data.clone();
            let tx_clone = tx.clone();
            task::spawn(async move {
                if let Err(e) = pkg::run(args, data_clone, tx_clone).await {
                    println!("Error parse package.json  {}", e);
                };
            });
        }
    }

    // web::run(Arc::clone(&package.pkg), package.sender.clone()).await;
    web::run(Arc::clone(&data.clone()), rx).await.unwrap();
}
