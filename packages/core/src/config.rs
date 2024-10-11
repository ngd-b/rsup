/// 解析配置文件
/// 操作配置文件
/// 默认目录：
/// macos|linux: /opt/rsup
/// windows: C:\\Program Files\\rsup
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub port: u16,
    pub dir: String,
}
