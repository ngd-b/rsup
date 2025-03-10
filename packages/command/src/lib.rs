use clap::Parser;

// 引入外部crate
extern crate config as external_config;
use config::Options as ConfigOptions;
use rs_utils;
use tokio::try_join;
use update::Options as UpdateOptions;

mod config;
mod update;

#[derive(Parser, Debug)]
pub enum Commands {
    #[clap(name = "config", about = "Manage the config file")]
    Config {
        #[clap(subcommand)]
        config: ConfigOptions,
    },
    #[clap(name = "update", about = "Update the rsup binary and web client")]
    Update {
        #[clap(subcommand)]
        update: Option<UpdateOptions>,
    },
}

/// 命令行交互
pub async fn run() {
    let cli = Commands::parse();

    let _ = match cli {
        Commands::Config { config } => match config {
            ConfigOptions::List => ConfigOptions::list_config().await,
            ConfigOptions::Set { key, value } => ConfigOptions::set_config_value(&key, value).await,
            ConfigOptions::Get { key } => ConfigOptions::get_config_value(&key).await,
            ConfigOptions::Delete => todo!(),
        },
        Commands::Update { update } => {
            // 获取最新的包地址
            let (rsup_url, rsup_web_url) = rs_utils::get_pkg_url(None);

            // 获取命令安装目录
            let config = external_config::Config::get_config().await;

            // 更新的包名
            match update {
                Some(options) => match options {
                    UpdateOptions::Rsup => UpdateOptions::rsup_update(rsup_url, &config.dir).await,
                    UpdateOptions::Web => {
                        UpdateOptions::rsup_web_update(rsup_web_url, &config.dir).await
                    }
                },
                None => {
                    // 两个都更新
                    let rsup = UpdateOptions::rsup_update(rsup_url, &config.dir);
                    let web = UpdateOptions::rsup_web_update(rsup_web_url, &config.dir);

                    match try_join!(rsup, web) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                }
            }
        }
    };
}

///
/// 显示版本信息
///
///
pub async fn show_version() {
    let config = external_config::Config::get_config().await;

    println!("{}", config.version);
    println!("web: {}", config.web.version);
}
