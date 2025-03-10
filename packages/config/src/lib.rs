//！Rsup 配置文件
//！
//! 读取配置文件
//! 操作配置文件
//! 默认目录：
//! macos|linux: /opt/rsup
//! windows: C:\\Program Files\\rsup
//!
//! 配置文件：
//! config.toml
//!
//! 配置文件格式：
//! ```toml
//! name = "rsup"
//! version = "0.1.0"
//! dir = "/opt/rsup"
//! [web]
//! port = 8888
//! static_dir = "/opt/rsup/web"
//!
//! [pkg]
//! npm_registry = "https://registry.npmmirror.com"
//! ```
//!

use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// 配置共享全局变量,避免重复读取配置文件
///
pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    // 这里调用初始化
    let config = Config::read_config().unwrap();

    RwLock::new(config)
});

/// 配置文件结构体
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub dir: String,
    // web 服务配置
    pub web: WebConfig,
    // 包管理配置
    pub pkg: PkgConfig,
}

/// web 配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebConfig {
    pub version: String,
    pub port: u16,
    pub static_dir: String,
}

/// 包管理配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgConfig {
    pub npm_registry: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "rsup".to_string(),
            version: "".to_string(),
            dir: Default::default(),
            web: WebConfig {
                version: "".to_string(),
                port: 8888,
                static_dir: Default::default(),
            },
            pkg: PkgConfig {
                npm_registry: "https://registry.npmmirror.com".to_string(),
            },
        }
    }
}

impl Config {
    /// 读取配置文件
    ///
    /// Returns:
    /// - `Result<Config, Box<dyn std::error::Error>>`
    ///
    fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
        // 读取配置文件
        let config_dir = Config::get_url();
        let config_file_dir = format!("{}/config.toml", config_dir);

        if !Path::new(&config_file_dir).exists() {
            let msg = format!("配置文件不存在，请先初始化配置文件: rsup init");

            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                msg,
            )));
        }
        let config_content = fs::read_to_string(&config_file_dir)?;

        let config: Config = toml::from_str(&config_content)?;

        // 保存配置数据共享
        // CONFIG.set(config.clone()).unwrap();
        Ok(config)
    }
    /// 写入配置文件
    ///
    /// Returns:
    /// - `Result<Config, Error>`
    ///

    pub async fn write_config() -> Result<Config, Error> {
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

        let mut config = Config::default();
        // 配置文件路径
        config.dir = config_dir.clone();
        // 静态文件目录
        config.web.static_dir = format!("{}/web", &config_dir);
        // rsup 版本
        match rsup_utils::get_version("rsup".to_string(), Option::None).await {
            Ok(version) => {
                config.version = version;
            }
            Err(e) => {
                eprintln!("获取rsup版本失败:{}", e)
            }
        }
        // 获取rsup-web 版本
        match rsup_utils::get_version("rsup-web".to_string(), Option::None).await {
            Ok(version) => {
                config.web.version = version;
            }
            Err(e) => {
                eprintln!("获取rsup-web版本失败:{}", e)
            }
        }

        let config_content = toml::to_string(&config)?;
        file.write_all(config_content.as_bytes())?;

        Ok(config)
    }

    /// 父级包获取配置
    pub async fn get_config() -> RwLockReadGuard<'static, Config> {
        CONFIG.read().await
    }
    /// 可更新配置
    pub async fn get_mut_config() -> RwLockWriteGuard<'static, Config> {
        CONFIG.write().await
    }
    /// 根据不同系统生成不同的配置文件路径
    ///
    /// Returns:
    /// - `String`
    ///
    pub fn get_url() -> String {
        match std::env::consts::OS {
            "windows" => "C:\\Program Files\\rsup".to_string(),
            _ => "/opt/rsup".to_string(),
        }
    }
    /// 获取配置信息
    ///
    /// Returns:
    /// - `Option<String>`
    ///
    pub fn get(&self, key: &str) -> Option<String> {
        let mut parts: Vec<&str> = key.split(".").collect();

        // 取值
        let key = parts.remove(0);

        match key {
            "name" => Some(self.name.clone()),
            "version" => Some(self.version.clone()),
            "dir" => Some(self.dir.clone()),
            "web" => self.web.get(parts.clone()),
            "pkg" => self.pkg.get(parts.clone()),
            _ => None,
        }
    }
    /// 设置配置信息
    pub fn set(&mut self, key: &str, value: String) -> Result<(), Error> {
        let mut parts: Vec<&str> = key.split(".").collect();

        // 取值
        let key = parts.remove(0);

        let bool = match key {
            // "name" => {
            //     self.name = value;
            //     true
            // }
            // "version" => {
            //     self.version = value;
            //     true
            // }
            // "dir" => {
            //     self.dir = value;
            //     true
            // }
            "web" => self.web.set(parts.clone(), value),
            "pkg" => self.pkg.set(parts.clone(), value),
            _ => return Err(anyhow!(format!("{} 配置项不可更改", key))),
        };
        if bool {
            // 更新配置文件
            let config_dir = Config::get_url();
            let config_file_dir = format!("{}/config.toml", config_dir);
            let config_content = toml::to_string(&self)?;
            fs::write(config_file_dir, config_content)?;

            Ok(())
        } else {
            Err(anyhow!("配置项不存在"))
        }
    }
}

/// web 配置
/// 获取配置信息
///
impl WebConfig {
    pub fn get(&self, mut parts: Vec<&str>) -> Option<String> {
        if parts.is_empty() {
            return None;
        }
        let key = parts.remove(0);

        match key {
            "port" => Some(self.port.clone().to_string()),
            "static_dir" => Some(self.static_dir.clone()),
            _ => None,
        }
    }
    pub fn set(&mut self, mut parts: Vec<&str>, value: String) -> bool {
        if parts.is_empty() {
            return false;
        }
        let key = parts.remove(0);
        match key {
            "port" => self.port = value.parse().unwrap(),
            "static_dir" => self.static_dir = value,

            _ => return false,
        };
        true
    }
}

/// 包管理配置
///
impl PkgConfig {
    pub fn get(&self, mut parts: Vec<&str>) -> Option<String> {
        if parts.is_empty() {
            return None;
        }
        let key = parts.remove(0);

        match key {
            "npm_registry" => Some(self.npm_registry.clone()),
            _ => None,
        }
    }
    pub fn set(&mut self, mut parts: Vec<&str>, value: String) -> bool {
        if parts.is_empty() {
            return false;
        }
        let key = parts.remove(0);
        match key {
            "npm_registry" => self.npm_registry = value,

            _ => return false,
        };
        true
    }
}
