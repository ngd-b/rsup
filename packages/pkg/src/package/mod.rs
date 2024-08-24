use std::collections::HashMap;

pub mod package_info;
pub mod package_json;

use package_info::PkgInfo;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pkg {
    pub path: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub scripts: HashMap<String, String>,
    pub dependencies: HashMap<String, PkgInfo>,
    pub dev_dependencies: HashMap<String, PkgInfo>,
}

impl Pkg {
    pub fn new() -> Self {
        Pkg {
            path: String::new(),
            name: None,
            version: None,
            description: None,
            scripts: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        }
    }
    /// 生成依赖数据对象
    pub fn generate_pkg_info(data: Option<HashMap<String, String>>) -> HashMap<String, PkgInfo> {
        let mut res = HashMap::new();

        if let Some(dep) = data {
            for (key, value) in dep {
                let mut dep_info: PkgInfo = Default::default();

                dep_info.name = key.clone();
                dep_info.version = Some(value);
                res.insert(key, dep_info);
            }
        }

        res
    }
}
