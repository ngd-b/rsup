/// 用户交互
/// 1. 选择下载源
/// 2. 是否添加命令到环境变量
///
///
use clap::{Parser, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::Write;
use std::path::Path;
use std::{error::Error, fs::OpenOptions};

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

/// 提示用户选择下载源
/// 2024-12-24 废弃，直接从github下载，不再提示用户选择
/// @return 下载源
pub fn _prompt_origin() -> Origin {
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

/// 提示用户是否添加命令到环境变量
/// 默认添加
pub fn prompt_add_to_env(path: &str) -> Result<(), Box<dyn Error>> {
    let os = std::env::consts::OS;

    if os == "windows" {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not support windows",
        )));
    }
    let select = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Add command to environment variable?")
        .default(true)
        .show_default(false)
        .interact()
        .unwrap();

    if select {
        let home_dir = std::env::var("HOME")?;

        // 确定系统使用的shell
        let shell_file_name = match os {
            "macos" => ".zshrc",
            _ => ".bashrc",
        };
        // 环境变量配置目录
        let shell_config_path = format!("{}/{}", home_dir, shell_file_name);

        if !Path::new(&shell_config_path).exists() {
            let msg = format!(
                "{} not found, you can create it manually",
                shell_config_path
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                msg,
            )));
        }
        println!("The shell config path: {}", shell_config_path);

        // 判断是否已经添加过
        let content = std::fs::read_to_string(&shell_config_path)?;
        if content.contains(&path) {
            println!("Already added to environment variable.");
        } else {
            // 写入配置
            let mut file = OpenOptions::new().append(true).open(shell_config_path)?;
            writeln!(file, "\n# Add rsup to PATH")?;
            writeln!(file, "export PATH=\"{}:$PATH\"", path)?;

            println!();
            println!("💯 Add command to environment variable successfully.");
            println!(
                "😀 Please run `source ~/{}` to take effect.",
                shell_file_name
            );
        }

        Ok(())
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "You can add it by yourself",
        )));
    }
}
