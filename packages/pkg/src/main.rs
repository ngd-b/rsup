use clap::Parser;
use pkg::{package, run, Args};
use tokio::{self};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // let data = Arc::new(Mutex::new(package::Pkg::new()));
    // let (tx, rx) = channel(100);

    let package = package::Package::new();

    // let data_clone = data.clone();
    let package_clone = package.clone();

    tokio::task::spawn(async move { run(args, package_clone).await });

    let mut rx = package.receiver.lock().await;
    loop {
        if let Some(_) = rx.recv().await {
            let pkg = package.get_pkg().await.clone();
            println!("recive data : {:#?}", pkg)
        };
    }
}
