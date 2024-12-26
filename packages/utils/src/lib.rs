use std::{error::Error, fs::File, path::Path};

use flate2::read::GzDecoder;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tar::Archive;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Clone)]
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
}

// 固定版本信息，
const VERSION: &str = "latest";

/// 根据系统获取rsup、rsup-web下载的地址
///
/// # Arguments:
/// * os 操作系统
/// * origin 下载源
///
/// # Returns
/// * rsup 下载地址
/// * rsup-web 下载地址
///
pub fn get_pkg_url(origin: Option<Origin>) -> (String, String) {
    let os = std::env::consts::OS;

    let origin = origin.unwrap_or(Origin::Github);
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
/// # Arguments
/// * url 下载地址
/// * target_dir 保存目录
///
/// # Returns
/// * Result<(), Box<dyn Error>>
///     
///     解压成功返回Ok(())
///
///     解压失败返回Err(Box<dyn Error>)
///
pub async fn decompress_file(url: &str, target_dir: &str) -> Result<(), Box<dyn Error>> {
    let tar_gz = File::open(url)?;

    let decomppress = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decomppress);

    // 处理解压目录，不存在则创建目录
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).await?;
    };

    archive.unpack(target_dir)?;

    Ok(())
}
/// 下载文件
/// 解压文件到指定目录
/// # Arguments
/// * client 请求客户端
/// * url 下载地址
/// * output 保存目录
///
/// # Returns
/// * Result<(), Box<dyn Error>>
///
///     下载成功返回Ok(())
///
///     下载失败返回Err(Box<dyn Error>)
///
pub async fn download_file(client: &Client, url: &str, output: &str) -> Result<(), Box<dyn Error>> {
    // 下载地址
    let res = client.get(url).send().await?;

    if res.status().is_success() {
        // 获取文件大小
        let content_size = res.content_length().ok_or("无法获取文件大小")?;

        // 下载成功
        // 保存文件到指定目录
        // 文件路径
        let mut file = fs::File::create(output).await?;

        // 创建进度条
        let pb = ProgressBar::new(content_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] {bar:80} {percent}%")?
                .progress_chars("##-"),
        );

        // 创建流式响应体
        let mut downloaded = 0;
        let mut stream = res.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).await?;

            let len = chunk.len() as u64;
            downloaded += len;
            pb.set_position(downloaded);
        }
        pb.finish_with_message("下载完成");
        // 保存文件
        // let bytes = res.bytes().await?;
        // file.write_all(&bytes).await?;
        Ok(())
    } else {
        let error_message = format!("Request failed with status code: {}", res.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        )))
    }
}
