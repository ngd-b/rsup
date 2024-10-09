use std::error::Error;

use clap::{command, Parser, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::Client;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::config::Config;

mod config;
#[derive(Parser, Debug, Clone)]
#[command(name = "rsup-installer", author = "hboot", version, about)]
pub struct Cli {
    #[arg(short, long, help = "选择下载源地址")]
    pub origin: Option<Origin>,
}
#[derive(Parser, Debug, Clone, ValueEnum)]
pub enum Origin {
    Github,
    Gitee,
}
impl Origin {
    pub fn get_pkg_url(&self) -> String {
        match self {
            Origin::Github => String::from("https://github.com/ngd-b"),
            Origin::Gitee => String::from("https://gitee.com/hboot"),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Origin::Github => "github",
            Origin::Gitee => "gitee",
        }
    }
    pub fn choices() -> Vec<&'static str> {
        vec![Origin::Github.as_str(), Origin::Gitee.as_str()]
    }
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

/// 提示用户选择下载源
/// @return 下载源
fn prompt_origin() -> Origin {
    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select download source...")
        .default(0)
        .items(Origin::choices().as_slice())
        .interact()
        .unwrap();

    match select {
        0 => Origin::Github,
        1 => Origin::Gitee,
        _ => unreachable!(),
    }
}

/// 下载文件
/// 解压文件到指定目录
async fn download_file(client: &Client, url: String, output: String) -> Result<(), Box<dyn Error>> {
    // 下载地址
    let res = client.get(url).send().await?.error_for_status()?;

    // 文件路径
    let mut file = fs::File::create(output).await?;

    // 保存文件
    let bytes = res.bytes().await?;
    file.write_all(&bytes).await?;

    Ok(())
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

    // 配置文件
    let config = Config::new().unwrap();

    println!("rsup will be installed in: {}", &config.dir);

    // 获取下载地址
    let (url, web_url) = get_pkg_url_by_os(origin);

    println!("rsup下载地址：{}", url);
    println!("rsup-web下载地址：{}", web_url);

    // 创建客户端
    let client = Client::new();
    // 下载rsup
    let rsup_task = download_file(&client, url.clone(), format!("{}/rsup.tar.gz", &config.dir));
    // 下载rsup-web
    let web_task = download_file(
        &client,
        web_url.clone(),
        format!("{}/rsup-web.tar.gz", &config.dir),
    );

    let (rsup_res, web_res) = tokio::join!(rsup_task, web_task);

    if rsup_res.is_err() {
        eprintln!("rsup下载失败 {},下载地址：{}", rsup_res.err().unwrap(), url);
    } else {
        println!("rsup下载成功");
    }

    if web_res.is_err() {
        eprintln!(
            "rsup-web下载失败：{},下载地址: {}",
            web_res.err().unwrap(),
            web_url
        );
    } else {
        println!("rsup-web下载成功");
    }
}
