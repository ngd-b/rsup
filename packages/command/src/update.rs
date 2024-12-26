use std::error::Error;

use clap::Parser;
use reqwest::Client;
use tokio::fs;
use utils;

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
        let rsup_url = format!("{}/rsup.tar.gz", dir);

        println!("正在更新rsup命令包...");
        println!("下载地址: {}", &url);
        println!("下载目录: {}", &rsup_url);
        println!("正在下载...");
        // 下载文件
        match utils::download_file(&client, &url, &rsup_url).await {
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
        match utils::decompress_file(&rsup_url, &dir).await {
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
        fs::remove_file(rsup_url).await?;
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
        match utils::download_file(&client, &url, &web_url).await {
            Ok(_) => {
                println!("下载完成");
            }
            Err(e) => {
                eprintln!("rsup下载失败 {}", e);
                return Err(e);
            }
        };

        println!("正在解压...");
        // 解压文件
        match utils::decompress_file(&web_url, &format!("{}/web", &dir)).await {
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
