use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, path::Path};
mod npm_lock;
mod pnpm_lock;
mod yarn_lock;

pub trait PkgLock: Send + Sync {
    fn new(file_path: String) -> Self
    where
        Self: Sized;
    fn read_pkg_graph(&self, name: String) -> Result<PkgInfo, Box<dyn Error>>;
    fn get_data(&self) -> LockPkg;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LockPkg {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: i32,
    #[serde(default)]
    pub packages: HashMap<String, PkgInfo>,
}

/**
 * 包管理器类型
 */
pub enum ManagerType {
    Npm,
    Yarn,
    Pnpm,
}
impl ManagerType {
    pub fn to_string(&self) -> String {
        match self {
            ManagerType::Npm => "npm".to_string(),
            ManagerType::Yarn => "yarn".to_string(),
            ManagerType::Pnpm => "pnpm".to_string(),
        }
    }
    pub fn from_str(s: &str) -> ManagerType {
        match s {
            "npm" => ManagerType::Npm,
            "yarn" => ManagerType::Yarn,
            "pnpm" => ManagerType::Pnpm,
            _ => ManagerType::Npm,
        }
    }
}
// 依赖关系结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgInfo {
    // 依赖名称
    #[serde(default)]
    pub name: String,
    // 依赖版本号
    #[serde(default)]
    pub version: String,
    // 依赖解析目标地址
    #[serde(default)]
    pub resolved: String,
    // 依赖完整性校验码
    #[serde(default)]
    pub integrity: Option<String>,
    // 是否为开发依赖
    #[serde(default)]
    pub dev: Option<bool>,
    // 是否有安装脚本
    #[serde(default, rename = "hasInstallScript")]
    pub has_install_script: Option<bool>,
    // 是否有shrinkwrap文件
    #[serde(default, rename = "hasShrinkwrap")]
    pub has_shrinkwrap: Option<bool>,
    #[serde(default, rename = "peerDependencies")]
    pub peer_dependencies: HashMap<String, String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
    #[serde(default)]
    pub is_peer: bool,
    // 是否相互依赖
    #[serde(default)]
    pub is_loop: bool,
    // 依赖关系
    #[serde(default)]
    pub relations: Vec<PkgInfo>,
    // 依赖路径
    #[serde(default)]
    pub path: String,
}

impl Default for PkgInfo {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Default::default(),
            resolved: Default::default(),
            integrity: Default::default(),
            dev: Default::default(),
            has_install_script: Default::default(),
            has_shrinkwrap: Default::default(),
            peer_dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            dependencies: HashMap::new(),
            is_peer: false,
            is_loop: false,
            relations: vec![],
            path: String::new(),
        }
    }
}

/**
 * 不同的包管理器对应的锁文件解析器
 *
 */
pub fn pkg_lock(name: String, file_path: String) -> Box<dyn PkgLock> {
    let manage_type = ManagerType::from_str(&name);
    match manage_type {
        ManagerType::Npm => Box::new(npm_lock::Pkg::new(file_path)),
        ManagerType::Yarn => Box::new(yarn_lock::Pkg::new(file_path)),
        ManagerType::Pnpm => Box::new(pnpm_lock::Pkg::new(file_path)),
    }
}
/**
 * 根据当前项目下的文件夹，判断当前使用的包管理工具
 */
pub fn get_current_manager(dir: &Path) -> Option<String> {
    if dir.join("package-lock.json").exists() {
        Some(ManagerType::Npm.to_string())
    } else if dir.join("pnpm-lock.yaml").exists() {
        Some(ManagerType::Pnpm.to_string())
    } else if dir.join("yarn.lock").exists() {
        Some(ManagerType::Yarn.to_string())
    } else {
        None
    }
}
