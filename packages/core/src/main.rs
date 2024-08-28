use clap::Parser;
use pkg::package::Package;

use tokio::task;
use web;
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

    // let data: Arc<Mutex<pkg::Pkg>> = Arc::new(Mutex::new(pkg::Pkg::new()));

    // let (tx, rx) = channel(100);

    let package = Package::new();
    // 默认启动pkg解析服务

    // let data_clone = data.clone();
    // let tx_clone = tx.clone();

    let package_clone = package.clone();
    task::spawn(async move {
        pkg::run(args.pkg_args, package_clone).await;
    });
    // 开启线程，需要处理线程使用异步运行时
    // let _ = task::spawn_blocking(move || pkg::run(args.pkg_args, data_clone, tx_clone));

    web::run(package.clone()).await;
}
