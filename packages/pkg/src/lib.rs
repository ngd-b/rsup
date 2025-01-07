//! pkg is a simple CLI tool for checking package versions.
//!
//! it will read the package.json file and check if there are any outdated dependencies
//!

use std::fs;
use std::path::Path;

use clap::Parser;
// use futures_util::{stream, StreamExt};
use package::package_info::{compare_version, fetch_pkg_info};
use package::package_json::read_pkg_json;
use package::{Package, Pkg};
pub mod package;
use config::Config;
use reqwest::Client;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "The path to the package.json file"
    )]
    pub dir: String,
}

/// check if the file exist
pub fn check_file_exist<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
/// the fun used to run the program
///
/// # example
/// ```
///
/// pkg::run(pkg::Args { dir: "." });
///
/// ```
pub async fn run(args: Args, package: Package) {
    let mut file_path = args.dir.clone();
    // 判断是不是一个文件夹目录
    let dir = Path::new(&args.dir);

    if dir.is_dir() {
        file_path.push_str("/package.json");
    } else {
        // 是一个文件路径，但是不是package.json文件
        if !args.dir.ends_with("package.json") {
            let parent_dir = dir.parent().unwrap();
            file_path = parent_dir
                .join("package.json")
                .to_str()
                .unwrap()
                .to_string();
        }
    }

    let pkg_file_path = Path::new(&file_path);

    if !check_file_exist(pkg_file_path) {
        // 错误消息提示

        panic!("The path '{}' does not exist.", &file_path)
    }
    // 从配置信息中读取npm配置信息
    let config = Config::get_config().await;
    let regitry = config.pkg.npm_registry.clone();
    match read_pkg_json(&pkg_file_path) {
        Ok(pkg) => {
            {
                // 用完即销毁
                let mut res = package.pkg.lock().await;
                res.path = pkg_file_path.to_str().unwrap().to_string();
                res.name = pkg.name;
                res.version = pkg.version;
                res.description = pkg.description;
                res.dependencies = Pkg::generate_pkg_info(pkg.dependencies.clone());
                res.dev_dependencies = Pkg::generate_pkg_info(pkg.dev_dependencies.clone());
                // 数据更新就通知
                package.sender.lock().await.send(()).await.unwrap();
            }

            // let mut tasks = Vec::new();

            let client = Client::new();

            let create_task = |name: String,
                               version: String,
                               client: Client,
                               data: Package,
                               is_dev,
                               regitry: String| {
                tokio::spawn(async move {
                    println!("Starting task for package: {}", name);
                    let info = fetch_pkg_info(&client, &name, &regitry).await.unwrap();

                    let mut new_info = info.clone();
                    let versions = compare_version(&version, &info.dist_tags.latest, info.versions);

                    new_info.version = Some(version.clone());
                    new_info.versions = versions;
                    new_info.is_finish = true;

                    println!("finish fetch pkg info for:{}", name);
                    {
                        let mut res = data.pkg.lock().await;
                        if is_dev {
                            res.dev_dependencies.insert(name.clone(), new_info);
                        } else {
                            res.dependencies.insert(name.clone(), new_info);
                        }
                        if let Err(e) = data.sender.lock().await.send(()).await {
                            eprintln!("Error sending update signal: {}", e);
                        };
                    }

                    println!("Completed task for package: {}", name);
                });
            };
            if let Some(dev_dep) = pkg.dev_dependencies {
                for (name, version) in dev_dep.iter() {
                    let _task = create_task(
                        name.to_string(),
                        version.to_string(),
                        client.clone(),
                        package.clone(),
                        true,
                        regitry.clone(),
                    );
                    // tasks.push(task);
                }
            }
            if let Some(dep) = pkg.dependencies {
                // 提前展示依赖名称
                for (name, version) in dep.iter() {
                    let _task = create_task(
                        name.to_string(),
                        version.to_string(),
                        client.clone(),
                        package.clone(),
                        false,
                        regitry.clone(),
                    );
                    // tasks.push(task);
                }
            }

            // let _ = stream::iter(tasks).for_each_concurrent(1, |task| task);
        }
        Err(e) => eprintln!("Error reading package.json: {}", e),
    };

    // Ok(())
}
