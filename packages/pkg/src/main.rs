use std::sync::Arc;

use clap::Parser;
use pkg::{run, Args};
use tokio::{
    self,
    sync::{mpsc::channel, Mutex},
};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let data = Arc::new(Mutex::new(pkg::Pkg::new()));
    let (tx, rx) = channel(100);

    let data_clone = data.clone();

    tokio::task::spawn(async move { run(args, data_clone, tx).await });

    let mut rx = rx;
    loop {
        println!("1");
        if let Some(_) = rx.recv().await {
            println!("recive data : {:#?}", data)
        };
    }
}
