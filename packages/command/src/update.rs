use std::{
    error::Error,
    path::{Path, PathBuf},
};

use clap::Parser;
use reqwest::Client;
use rs_utils;
use tokio::fs;

#[derive(Parser, Debug)]
pub enum Options {
    Rsup,
    Web,
}

impl Options {
    /// 更新rsup命令包
    pub async fn rsup_update(url: String, dir: &str) -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        // 下载目录
        let rsup_file = PathBuf::from(dir).join("rsup.tar.gz");
        let rsup_url = rsup_file.to_string_lossy().to_string();

        println!("正在更新rsup命令包...");
        println!("下载地址: {}", &url);
        println!("下载目录: {}", &rsup_url);
        println!("正在下载...");
        // 下载文件
        match rs_utils::fs::download_file(&client, &url, &rsup_url).await {
            Ok(_) => {
                println!("下载完成");
            }
            Err(e) => {
                eprintln!("rsup下载失败 {}", e);
                return Err(e);
            }
        }
        println!("正在解压...");
        // 解压文件
        match rs_utils::fs::decompress_file(&rsup_url, &dir).await {
            Ok(_) => {
                println!("解压完成");
            }
            Err(e) => {
                eprintln!("rsup解压失败 {}", e);
                return Err(e);
            }
        }
        println!("正在清理...");

        // 删除文件
        if let Err(e) = fs::remove_file(&rsup_url).await {
            eprintln!("{}文件删除失败 {}", &rsup_url, e);
            return Err(Box::new(e));
        };
        println!("清理完成");
        println!("更新完成");
        println!("🥰 Enjoy!");
        Ok(())
    }
    /// 更新web服务资源
    ///
    pub async fn rsup_web_update(url: String, dir: &str) -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        // 下载目录
        let web_url = format!("{}/rsup-web.tar.gz", &dir);

        println!("正在更新rsup-web命令包...");
        println!("下载地址: {}", &url);
        println!("下载目录: {}", &web_url);
        println!("正在下载...");
        // 下载文件
        match rs_utils::fs::download_file(&client, &url, &web_url).await {
            Ok(_) => {
                println!("下载完成");
            }
            Err(e) => {
                eprintln!("rsup下载失败 {}", e);
                return Err(e);
            }
        };

        println!("正在解压...");
        // 删除旧文件
        let target_dir = format!("{}/web", &dir);
        if Path::new(&target_dir).exists() {
            fs::remove_dir_all(&target_dir).await?;
        }
        // 解压文件
        match rs_utils::fs::decompress_file(&web_url, &target_dir).await {
            Ok(_) => {
                println!("解压完成");
            }
            Err(e) => {
                eprintln!("rsup解压失败 {}", e);
                return Err(e);
            }
        }
        println!("正在清理...");
        // 删除文件
        fs::remove_file(web_url).await?;
        println!("清理完成");
        println!("更新完成");
        println!("🥰 Enjoy!");
        Ok(())
    }
}
