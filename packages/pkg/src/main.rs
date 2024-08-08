use std::sync::{mpsc::channel, Arc};

use clap::Parser;
use pkg::{run, Args};
use tokio::{self, sync::Mutex};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let data = Arc::new(Mutex::new(pkg::Pkg::new()));
    let (tx, rx) = channel();

    let data_clone = data.clone();
    match run(args, data_clone, tx).await {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(e) => {
            eprintln!("Error reading package.json: {}", e)
        }
    };

    while let Ok(_) = rx.recv() {
        println!("recive data : {:#?}", data)
    }
}
