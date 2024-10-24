use std::{
    error::Error,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use once_cell::sync::OnceCell;
/// 解析配置文件
/// 操作配置文件
/// 默认目录：
/// macos|linux: /opt/rsup
/// windows: C:\\Program Files\\rsup
use serde_derive::{Deserialize, Serialize};
// 全局共享配置数据

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub dir: String,
    // web 服务配置
    pub web: WebConfig,
    // 包管理配置
    pub pkg: PkgConfig,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub port: u16,
    pub static_dir: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PkgConfig {
    pub npm_registry: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "rsup".to_string(),
            version: "0.3.0".to_string(),
            dir: Default::default(),
            web: WebConfig {
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
    pub async fn read_config() -> Result<(), Box<dyn Error>> {
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
        CONFIG.set(config).unwrap();
        Ok(())
    }
    /// 写入配置文件
    pub async fn write_config() -> Result<Config, Box<dyn Error>> {
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

        let config_content = toml::to_string(&config)?;
        file.write_all(config_content.as_bytes())?;

        Ok(config)
    }

    /// 父级包获取配置
    pub fn get_config() -> &'static Config {
        CONFIG.get().unwrap()
    }
    /// 根据不同系统生成不同的配置文件路径
    pub fn get_url() -> String {
        match std::env::consts::OS {
            "windows" => "C:\\Program Files\\rsup".to_string(),
            _ => "/opt/rsup".to_string(),
        }
    }
    /// 获取配置信息
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
}

/// web 配置
/// 获取配置信息
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
}

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
}
