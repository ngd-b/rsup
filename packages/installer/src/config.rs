use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use serde_derive::{Deserialize, Serialize};
/// 配置文件
/// 初次安装需要生成一个配置文件
/// 配置文件路径为当前系统目录下的
///
///

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub port: u16,
    pub dir: String,
}

impl Config {
    /// 生成配置文件
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let config_dir = Config::get_url();

        // 创建配置文件目录
        if !Path::new(&config_dir).exists() {
            match fs::create_dir(&config_dir) {
                Ok(_) => {}
                Err(e) => {
                    if e.kind() == io::ErrorKind::PermissionDenied {
                        // 权限不足，
                        eprintln!("无权限访问，请使用管理员权限访问:{}", e)
                    } else {
                        eprintln!("创建配置文件目录失败:{}", e)
                    }
                    std::process::exit(1);
                }
            };
        }
        // 配置文件
        let config_url = format!("{}/config.toml", config_dir);
        let mut file = File::create(config_url.clone())?;

        let config = Config {
            version: "".to_string(),
            port: 8888,
            dir: config_dir,
        };
        let config_content = toml::to_string(&config)?;
        file.write_all(config_content.as_bytes())?;

        Ok(config)
    }
    /// 根据不同系统生成不同的配置文件路径
    pub fn get_url() -> String {
        match std::env::consts::OS {
            "windows" => "C:\\Program Files\\rsup".to_string(),
            _ => "/opt/rsup".to_string(),
        }
    }
}
