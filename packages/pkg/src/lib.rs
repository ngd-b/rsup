//! pkg is a simple CLI tool for checking package versions.
//!
//! it will read the package.json file and check if there are any outdated dependencies
//!
use std::path::Path;

use clap::Parser;
use package::package_info::{compare_version, fetch_pkg_info};
use package::package_json::read_pkg_json;
mod package;
use reqwest::Client;

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
pub async fn run(args: Args) {
    let pkg_file_path = Path::new(&args.dir).join("package.json");

    match read_pkg_json(&pkg_file_path) {
        Ok(pkg) => {
            println!("Package name: {}", pkg.name.unwrap());
            println!("Version: {}", pkg.version.unwrap());

            //
            let client: Client = Client::new();
            if let Some(dev_dep) = pkg.dev_dependencies {
                for (name, version) in dev_dep.iter() {
                    println!("dev_dep: {} @ {}", name, version);

                    match fetch_pkg_info(&client, name).await {
                        Ok(info) => {
                            println!("Latest version: {:#?}", info.dist_tags.latest);
                            // 输出最新版本之间的版本
                            let versions =
                                compare_version(version, &info.dist_tags.latest, &info.versions);

                            println!("Versions between {:?}", versions);
                        }
                        Err(e) => {
                            println!("Error fetching info for {}: {}", name, e);
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading package.json: {}", e),
    }
}
