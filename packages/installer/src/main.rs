use std::error::Error;
use std::fs::File;
use std::path::Path;

use clap::{command, Parser};
use flate2::read::GzDecoder;
use reqwest::Client;
use tar::Archive;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::prompt::{prompt_add_to_env, prompt_origin, Origin};
use config::Config;

mod prompt;
#[derive(Parser, Debug, Clone)]
#[command(name = "rsup-installer", author = "hboot", version, about)]
pub struct Cli {
    #[arg(short, long, help = "选择下载源地址")]
    pub origin: Option<Origin>,
}

// 固定版本信息，
const VERSION: &str = "latest";

/// 根据系统获取rsup、rsup-web下载的地址
/// @param os 操作系统
/// @param origin 下载源
fn get_pkg_url_by_os(origin: Origin) -> (String, String) {
    let os = std::env::consts::OS;

    // 下载地址
    let mut url = format!(
        "{}/rsup/releases/download/{}",
        origin.get_pkg_url(),
        VERSION
    );
    let mut web_url = format!(
        "{}/rsup-web/releases/download/{}",
        origin.get_pkg_url(),
        VERSION
    );

    // 根据系统不同下载不同的包，
    // 后缀名不一样
    let file_suffix = match os {
        "windows" => "windows-latest",
        "macos" => "macos-latest",
        _ => "ubuntu-latest",
    };
    url = format!("{}/rsup-{}.tar.gz", url, file_suffix);
    web_url = format!("{}/rsup-web.tar.gz", web_url);

    (url, web_url)
}

/// 解压文件
///
/// @param url 下载地址
/// @param target_dir 保存目录
async fn decompress_file(url: &str, target_dir: &str) -> Result<(), Box<dyn Error>> {
    let tar_gz = File::open(url)?;

    let decomppress = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decomppress);

    // 处理解压目录，不存在则创建目录
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).await?;
    }
    archive.unpack(target_dir)?;

    Ok(())
}
/// 下载文件
/// 解压文件到指定目录
async fn download_file(client: &Client, url: &str, output: &str) -> Result<(), Box<dyn Error>> {
    // 下载地址
    let res = client.get(url).send().await?;

    if res.status().is_success() {
        // 下载成功
        // 保存文件到指定目录
        // 文件路径
        let mut file = fs::File::create(output).await?;

        // 保存文件
        let bytes = res.bytes().await?;
        file.write_all(&bytes).await?;
        Ok(())
    } else {
        let error_message = format!("Request failed with status code: {}", res.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        )))
    }
}

#[tokio::main]
async fn main() {
    // 解析命令行参数
    let args = Cli::parse();

    // 根据系统获取rsup、rsup-web下载的地址
    let origin = match args.origin {
        Some(o) => o,
        None => prompt_origin(),
    };

    // 创建配置文件
    let config = Config::write_config().unwrap();

    println!("rsup will be installed in: {}", &config.dir);

    // 获取下载地址
    let (url, web_url) = get_pkg_url_by_os(origin);

    println!();
    println!("rsup下载地址：{}", url);
    println!("rsup-web下载地址：{}", web_url);

    // 创建客户端
    let client = Client::new();
    // 下载rsup
    let rsup_url = format!("{}/rsup.tar.gz", &config.dir);
    let rsup_task = download_file(&client, &url, &rsup_url);
    // 下载rsup-web
    let rsup_web_url = format!("{}/rsup-web.tar.gz", &config.dir);
    let web_task = download_file(&client, &web_url, &rsup_web_url);

    let (rsup_res, web_res) = tokio::join!(rsup_task, web_task);

    if rsup_res.is_err() {
        eprintln!("rsup下载失败 {}", rsup_res.err().unwrap());
    } else {
        println!("rsup下载成功");
        // 解压文件
        match decompress_file(&rsup_url, &config.dir).await {
            Ok(_) => {
                println!("rsup解压成功,解压目录为：{}", &config.dir)
            }
            Err(e) => {
                eprintln!("rsup解压失败：{}", e);
            }
        }
    }

    if web_res.is_err() {
        eprintln!("rsup-web下载失败：{}", web_res.err().unwrap(),);
    } else {
        println!("rsup-web下载成功");
        // 解压文件
        let target_dir = format!("{}/web", &config.dir);
        match decompress_file(&rsup_web_url, &target_dir).await {
            Ok(_) => {
                println!("rsup-web解压成功,解压目录为：{}/web", &config.dir)
            }
            Err(e) => {
                eprintln!("rsup-web解压失败：{}", e);
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
    };
    // 确认
}
