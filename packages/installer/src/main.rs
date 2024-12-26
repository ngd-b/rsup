use clap::{command, Parser};
use reqwest::Client;
use tokio::fs;

use crate::prompt::{prompt_add_to_env, Origin};
use config::Config;
use utils;

mod prompt;
#[derive(Parser, Debug, Clone)]
#[command(name = "rsup-installer", author = "hboot", version, about)]
pub struct Cli {
    #[arg(short, long, help = "选择下载源地址")]
    pub origin: Option<Origin>,
}

#[tokio::main]
async fn main() {
    // 解析命令行参数
    // let args = Cli::parse();

    // 根据系统获取rsup、rsup-web下载的地址
    // let origin = match args.origin {
    //     Some(o) => o,
    //     None => prompt_origin(),
    // };

    // 2024-12-25 暂时不使用交互式选择下载源，直接从github上下载资源
    // 默认从github上下载资源
    // let origin = Origin::Github;

    // 创建配置文件
    let config = Config::write_config().unwrap();

    println!("rsup will be installed in: {}", &config.dir);

    // 获取下载地址
    let (url, web_url) = utils::get_pkg_url(None);

    println!();
    println!("rsup下载地址：{}", url);
    println!("rsup-web下载地址：{}", web_url);

    // 创建客户端
    let client = Client::new();
    // 下载rsup
    let rsup_url = format!("{}/rsup.tar.gz", &config.dir);
    let rsup_task = utils::download_file(&client, &url, &rsup_url);
    // 下载rsup-web
    let rsup_web_url = format!("{}/rsup-web.tar.gz", &config.dir);
    let web_task = utils::download_file(&client, &web_url, &rsup_web_url);

    let (rsup_res, web_res) = tokio::join!(rsup_task, web_task);

    if rsup_res.is_err() {
        eprintln!("rsup下载失败 {}", rsup_res.err().unwrap());
        // 退出
        return;
    } else {
        println!("rsup下载成功");
        // 解压文件
        match utils::decompress_file(&rsup_url, &config.dir).await {
            Ok(_) => {
                println!("rsup解压成功,解压目录为：{}", &config.dir);
                // 解压完删除文件
                fs::remove_file(&rsup_url).await.unwrap();
            }
            Err(e) => {
                eprintln!("rsup解压失败：{}", e);
                return;
            }
        }
    }

    if web_res.is_err() {
        eprintln!("rsup-web下载失败：{}", web_res.err().unwrap(),);
        return;
    } else {
        println!("rsup-web下载成功");
        // 解压文件
        let target_dir = format!("{}/web", &config.dir);
        match utils::decompress_file(&rsup_web_url, &target_dir).await {
            Ok(_) => {
                println!("rsup-web解压成功,解压目录为：{}/web", &config.dir);
                // 解压完删除文件
                fs::remove_file(&rsup_web_url).await.unwrap();
            }
            Err(e) => {
                eprintln!("rsup-web解压失败：{}", e);
                return;
            }
        }
    }

    println!();
    println!("rsup 可执行文件目录为：{}", &config.dir);
    let shell_path = format!("{}", &config.dir);

    match prompt_add_to_env(&shell_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            println!("You can add command to environment variable by yourself.");
        }
    }
    // 确认
}
