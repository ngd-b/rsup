use std::sync::Arc;

use clap::Parser;
use pkg::{package, run, Args};
use tokio::{
    self,
    sync::{mpsc::channel, Mutex},
};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let data = Arc::new(Mutex::new(package::Pkg::new()));
    let (tx, rx) = channel(100);

    let package = Arc::new(Mutex::new(package::Package {
        pkg: data,
        sender: tx,
    }));

    // let data_clone = data.clone();
    let package_clone = package.clone();

    tokio::task::spawn(async move { run(args, package_clone).await });

    let mut rx = rx;
    loop {
        if let Some(_) = rx.recv().await {
            let pkg = package.lock().await.pkg.lock().await.clone();
            println!("recive data : {:#?}", pkg)
        };
    }
}
