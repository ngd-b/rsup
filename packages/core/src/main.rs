use std::sync::{Arc, Mutex};

use clap::{Parser, Subcommand};
use package::Pakcage;
use pkg;
use tokio::sync::broadcast;
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

async fn update_data(args: pkg::Args, data: Arc<Mutex<pkg::Pkg>>, tx: broadcast::Sender<pkg::Pkg>) {
    match pkg::run(args).await {
        Ok(res) => {
            let mut data = data.lock().expect("Mutex lock failed");
            *data = res.into();
            if let Err(e) = tx.send(data) {
                println!("Failed to send data: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading package.json: {}", e)
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // let data = Arc::new(Mutex::new(pkg::Pkg::new()));

    // let (tx, _rx) = broadcast::channel(10);
    let package = Pakcage::new();

    match args.command {
        Commands::Pkg(args) => {
            let data_clone = Arc::clone(&package.pkg);

            let tx_clone = package.sender.clone();
            task::spawn(async move {
                update_data(args, data_clone, tx_clone).await;
            });
        }
    }
    web::run(Arc::clone(&package.pkg), package.sender.clone()).await;
}
