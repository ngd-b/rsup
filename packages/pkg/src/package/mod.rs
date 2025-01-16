use std::{collections::HashMap, error::Error, sync::Arc};

pub mod package_info;
pub mod package_json;

use package_info::PkgInfo;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pkg {
    pub path: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub scripts: HashMap<String, String>,
    // 当前项目的管理工具
    pub manager_name: Option<String>,
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
            manager_name: None,
            scripts: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        }
    }
    /// 更新某个依赖的版本信息
    pub async fn update_pkg_info(
        data: Arc<Mutex<Pkg>>,
        pkg_info: package_json::UpdateParams,
    ) -> Result<(), Box<dyn Error>> {
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
        new_info.version = Some(pkg_info.version);
        new_info.versions = versions;
        new_info.is_del = false;

        {
            // let mut res = res.lock().await;
            if pkg_info.is_dev {
                locked_data.dev_dependencies.insert(pkg_info.name, new_info);
            } else {
                locked_data.dependencies.insert(pkg_info.name, new_info);
            }
        };

        Ok(())
        // if let Err(e) = tx.send(()).await {
        //     eprintln!("Error sending update signal: {}", e);
        // };
    }
    /// 删除某个依赖
    ///
    /// 软删除，标记删除；还可以恢复
    ///
    pub async fn remove_pkg_info(
        data: Arc<Mutex<Pkg>>,
        pkg_info: package_json::RemoveParams,
    ) -> Result<(), Box<dyn Error>> {
        let mut locked_data = data.lock().await;

        let dep = if pkg_info.is_dev {
            //
            locked_data.dev_dependencies.clone()
        } else {
            locked_data.dependencies.clone()
        };
        // 查找name
        let mut new_info = dep.get(&pkg_info.name).unwrap().clone();
        // 标记删除
        new_info.is_del = true;
        {
            // let mut res = res.lock().await;
            if pkg_info.is_dev {
                locked_data.dev_dependencies.insert(pkg_info.name, new_info);
            } else {
                locked_data.dependencies.insert(pkg_info.name, new_info);
            }
        };

        Ok(())
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

/// 全局共享数据对象
///
///
pub struct Package {
    pub pkg: Arc<Mutex<Pkg>>,
    pub sender: Arc<Mutex<Sender<()>>>,
    pub receiver: Arc<Mutex<Receiver<()>>>,
}

impl Clone for Package {
    fn clone(&self) -> Self {
        Self {
            pkg: self.pkg.clone(),
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}
impl Package {
    pub fn new() -> Self {
        let (tx, rx) = channel(1000);

        Package {
            pkg: Arc::new(Mutex::new(Pkg::new())),
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
        }
    }
    pub async fn update_pkg(
        self,
        pkg_info: package_json::UpdateParams,
    ) -> Result<(), Box<dyn Error>> {
        // let pkg = self.pkg.lock().await;
        match Pkg::update_pkg_info(self.pkg.clone(), pkg_info).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub async fn remove_pkg(
        self,
        pkg_info: package_json::RemoveParams,
    ) -> Result<(), Box<dyn Error>> {
        // let pkg = self.pkg.lock().await;
        match Pkg::remove_pkg_info(self.pkg.clone(), pkg_info).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    /// 获取pkg 数据
    ///
    /// 非共享，纯数据
    pub async fn get_pkg(&self) -> Pkg {
        let data_lock = self.pkg.lock().await;

        data_lock.clone()
    }
}
