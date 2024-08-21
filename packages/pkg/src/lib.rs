//! pkg is a simple CLI tool for checking package versions.
//!
//! it will read the package.json file and check if there are any outdated dependencies
//!

use std::error::Error;
use std::path::Path;

use std::sync::Arc;

use clap::Parser;
use package::package_info::{compare_version, fetch_pkg_info};
use package::package_json::read_pkg_json;
pub use package::Pkg;
pub mod package;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub dir: String,
    #[arg(short, long)]
    pub list: bool,
}

/// the fun used to run the program
///
/// # example
/// ```
///
/// pkg::run(pkg::Args { dir: "." });
///
/// ```
pub async fn run(
    args: Args,
    data: Arc<Mutex<Pkg>>,
    tx: Sender<()>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let pkg_file_path = Path::new(&args.dir).join("package.json");

    // let mut res = Pkg::new();
    let mut res = data.lock().await;

    match read_pkg_json(&pkg_file_path) {
        Ok(pkg) => {
            (*res).name = pkg.name.unwrap();
            (*res).version = pkg.version.unwrap();
            (*res).description = pkg.description.unwrap();
            // 数据更新就通知
            tx.send(()).await.unwrap();

            let client = Client::new();
            if let Some(dev_dep) = pkg.dev_dependencies {
                for (name, version) in dev_dep.iter() {
                    match fetch_pkg_info(&client, &name).await {
                        Ok(info) => {
                            let mut new_info = info.clone();
                            // 输出最新版本之间的版本
                            let versions =
                                compare_version(version, &info.dist_tags.latest, info.versions);

                            new_info.version = Some(version.to_string());
                            new_info.versions = versions;
                            (*res).dev_dependencies.insert(name.to_string(), new_info);
                            // 数据更新就通知
                            tx.send(()).await.unwrap();
                        }
                        Err(e) => {
                            println!("Error fetching info for {}: {}", name, e);
                        }
                    };
                }
            }
            if let Some(dep) = pkg.dependencies {
                for (name, version) in dep.iter() {
                    match fetch_pkg_info(&client, &name).await {
                        Ok(info) => {
                            let mut new_info = info.clone();
                            // 输出最新版本之间的版本
                            let versions =
                                compare_version(version, &info.dist_tags.latest, info.versions);

                            new_info.version = Some(version.to_string());
                            new_info.versions = versions;
                            (*res).dev_dependencies.insert(name.to_string(), new_info);
                            // 数据更新就通知
                            tx.send(()).await.unwrap();
                        }
                        Err(e) => {
                            println!("Error fetching info for {}: {}", name, e);
                        }
                    };
                }
            }
        }
        Err(e) => eprintln!("Error reading package.json: {}", e),
    }

    Ok(())
}
