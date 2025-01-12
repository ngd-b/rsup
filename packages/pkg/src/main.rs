use std::error::Error;

use clap::Parser;
use pkg::{manager, package, run, Args};
use tokio::{self};

// 文件路径
fn file_exist(dir: String) -> String {
    let mut file_path = dir.clone();
    if !dir.ends_with("package.json") {
        file_path.push_str("/package.json");
    }

    file_path
}

/// 测试运行package 包
async fn run_package(args: Args) {
    let package = package::Package::new();

    // let data_clone = data.clone();
    let package_clone = package.clone();

    tokio::task::spawn(async move {
        run(args, package_clone).await;
    });

    let mut rx = package.receiver.lock().await;
    loop {
        if let Some(_) = rx.recv().await {
            let pkg = package.get_pkg().await.clone();
            println!("recive data : {:#?}", pkg)
        };
    }
}

fn run_package_lock(args: Args, name: String) -> Result<manager::PkgInfo, Box<dyn Error>> {
    let file_path = file_exist(args.dir.to_string());
    // package::package_lock::Pkg::read_pkg_graph(name, file_path)
    let mut pkg = manager::pkg_lock("npm", name, file_path);

    let pkg_info = pkg.read_pkg_graph().unwrap();

    Ok(pkg_info)
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    // 测试运行package包
    // run_package(args).await;

    // 测试读取package-lock.json文件
    // match run_package_lock(args, "vue".to_string()) {
    match run_package_lock(args, "unocss".to_string()) {
        Ok(pkg) => {
            println!("{:#?}", pkg)
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
