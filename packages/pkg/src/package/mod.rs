use std::{collections::HashMap, sync::Arc};

pub mod package_info;
pub mod package_json;

use package_info::PkgInfo;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::Mutex;

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
    /// 更新某个依赖的版本信息
    pub async fn update_pkg_info(data: Arc<Mutex<Pkg>>, pkg_info: package_json::UpdateParams) {
        let mut locked_data = data.lock().await;

        let dep = if pkg_info.is_dev {
            //
            locked_data.dev_dependencies.clone()
        } else {
            locked_data.dependencies.clone()
        };
        // 查找name

        let mut new_info = dep.get(&pkg_info.name).unwrap().clone();
        let versions = package_info::compare_version(
            &pkg_info.version,
            &new_info.dist_tags.latest,
            new_info.versions,
        );

        new_info.versions = versions;

        {
            // let mut res = res.lock().await;
            if pkg_info.is_dev {
                locked_data.dev_dependencies.insert(pkg_info.name, new_info);
            } else {
                locked_data.dependencies.insert(pkg_info.name, new_info);
            }
        }

        // if let Err(e) = tx.send(()).await {
        //     eprintln!("Error sending update signal: {}", e);
        // };
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
