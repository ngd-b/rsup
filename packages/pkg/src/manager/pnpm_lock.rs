use crate::manager::{PkgInfo, PkgLock};
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
/// 依赖关系图
/// 查询某个依赖的依赖关系图
///
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use super::{LockPkg, ManagerType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgSetting {
    #[serde(default)]
    #[serde(rename = "autoInstallPeers")]
    pub auto_install_peers: bool,
    #[serde(default)]
    #[serde(rename = "excludeLinksFromLockfile")]
    pub exclude_links_from_lockfile: bool,
}

impl Default for PkgSetting {
    fn default() -> Self {
        PkgSetting {
            auto_install_peers: true,
            exclude_links_from_lockfile: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgDependence {
    specifier: String,
    version: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgImporter {
    #[serde(default)]
    pub dependencies: HashMap<String, PkgDependence>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, PkgDependence>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pkg {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    #[serde(rename = "lockfileVersion")]
    pub lockfile_version: String,
    #[serde(default)]
    pub settings: PkgSetting,
    #[serde(default)]
    pub packages: HashMap<String, PkgInfo>,
    // pnpm 8
    #[serde(default)]
    pub dependencies: HashMap<String, PkgDependence>,
    // pnpm 8
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, PkgDependence>,
    // pnpm 9
    #[serde(default)]
    pub importers: HashMap<String, PkgImporter>,
    // pnpm9
    #[serde(default)]
    pub snapshots: HashMap<String, PkgInfo>,
    // #[serde(default)]
    // pub dep_name: String,
    // #[serde(default)]
    // pub pkg_info: PkgInfo,
}

impl Default for Pkg {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Default::default(),
            lockfile_version: Default::default(),
            settings: Default::default(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            importers: HashMap::new(),
            packages: HashMap::new(),
            snapshots: HashMap::new(),
            // dep_name: Default::default(),
            // pkg_info: PkgInfo::default(),
        }
    }
}

impl PkgLock for Pkg {
    fn new(file_path: String) -> Pkg {
        println!("开始从{}读取依赖关系...", file_path);
        // 根据版本应用不同的实例
        let pkg = Pkg::read_pkg(file_path.clone()).unwrap();
        println!("读取{}依赖关系完成", file_path);
        pkg
    }

    /// 读取某个依赖的依赖关系图
    fn read_pkg_graph(&self, dep_name: String) -> Result<PkgInfo, Box<dyn Error>> {
        // 如果当前npm版本很低，则不支持查询
        if self.lockfile_version != "6.0" && self.lockfile_version != "9.0" {
            return Err("当前pnpm版本不支持查询依赖关系图".into());
        }
        println!("当前pnpm-lock.json文件版本为{}", self.lockfile_version);
        let find_dependence = |(dependencies, dev_dependencies): (
            &HashMap<String, PkgDependence>,
            &HashMap<String, PkgDependence>,
        )|
         -> PkgDependence {
            if dependencies.contains_key(&dep_name) {
                dependencies.get(&dep_name).unwrap().clone()
            } else {
                dev_dependencies.get(&dep_name).unwrap().clone()
            }
        };

        let dependence = if self.lockfile_version == "6.0" {
            find_dependence((&self.dependencies, &self.dev_dependencies))
        } else {
            // 9.0
            let importer = if self.importers.contains_key(".") {
                self.importers.get(".").unwrap()
            } else {
                self.importers.get("").unwrap()
            };
            find_dependence((&importer.dependencies, &importer.dev_dependencies))
        };
        let key = self.get_package_key(&dep_name, &dependence.version);

        let mut pkg_info = match self.get_package_info(&key) {
            Some(info) => info,
            None => {
                return Err(format!("当前路径未找到依赖：{}", &dep_name).into());
            }
        };

        // 当前依赖名称设置为顶层路径的依赖名
        pkg_info.name = dep_name.clone();
        pkg_info.path = key.clone();
        pkg_info.version = dependence.version.clone();
        // 开始递归查找依赖关系图

        let relations = self
            .read_pkg_child_graph(pkg_info.clone(), vec![key])
            .unwrap();

        pkg_info.relations = relations;
        Ok(pkg_info)
    }

    fn get_data(&self) -> LockPkg {
        let v = self.lockfile_version.parse::<i32>().unwrap_or(0);
        LockPkg {
            name: ManagerType::Pnpm.to_string(),
            version: v,
            packages: self.packages.clone(),
        }
    }
}

impl Pkg {
    fn get_package_key(&self, name: &str, version: &str) -> String {
        match self.lockfile_version.as_str() {
            "6.0" => format!("/{}@{}", name, version),
            "9.0" => format!("{}@{}", name, version),
            _ => format!("/{}@{}", name, version),
        }
    }
    fn get_package_info(&self, key: &str) -> Option<PkgInfo> {
        match self.lockfile_version.as_str() {
            "6.0" => self.packages.get(key).cloned(),
            "9.0" => self.snapshots.get(key).cloned(),
            _ => None,
        }
    }
    // 读取的package.json文件
    fn read_pkg(file_path: String) -> Result<Pkg, Box<dyn Error>> {
        // 项目所在目录
        let path = Path::new(&file_path);

        let dir = path.parent().unwrap();
        // 读取package.json文件
        let pkg_path = dir.join("pnpm-lock.yaml");

        if !pkg_path.exists() {
            return Err("pnpm-lock.yaml文件不存在".into());
        }

        let file = File::open(pkg_path)?;
        let reader = BufReader::new(file);
        let package = serde_yaml::from_reader(reader)?;

        Ok(package)
    }

    fn read_pkg_child_graph(
        &self,
        parent: PkgInfo,
        visited: Vec<String>,
    ) -> Result<Vec<PkgInfo>, Box<dyn Error>> {
        let mut relations = Vec::new();

        let mut process_dependencies = |dependencies: &HashMap<String, String>, is_peer: bool| {
            for (child_name, child_version) in dependencies.iter() {
                // 递归查找依赖关系图
                let mut child = self
                    .read_pkg_graph_recursively(
                        child_name.to_string(),
                        child_version.to_string(),
                        visited.clone(),
                    )
                    .unwrap();

                child.is_peer = is_peer;
                relations.push(child);
            }
        };

        process_dependencies(&parent.dependencies, false);
        process_dependencies(&parent.peer_dependencies, true);

        Ok(relations)
    }
    /// 递归读取指定依赖的依赖项
    /// 优先从父级路径查找依赖，处理存在冲突依赖的问题；
    /// 通常所有依赖都会被提升到顶级路径，有公用依赖就不需要重复安装
    ///
    fn read_pkg_graph_recursively(
        &self,
        name: String,
        version: String,
        visited: Vec<String>,
    ) -> Result<PkgInfo, Box<dyn Error>> {
        let key = self.get_package_key(&name, &version);
        println!("开始递归读取依赖关系图,当前依赖：{:#?}", &key);
        // let mut graph = PkgInfo::default();

        let mut graph = match self.get_package_info(&key) {
            Some(info) => {
                println!("找到依赖：{}", &key);
                info
            }
            None => PkgInfo::default(),
        };
        graph.name = name.clone();
        graph.version = version.clone();
        graph.path = key.clone();

        if visited.contains(&key) {
            graph.is_loop = true;
            println!("存在循环依赖：{}", &key);
            return Ok(graph);
        }
        let mut visited = visited.clone();
        visited.push(key.clone());
        // 递归处理依赖关系图
        graph.relations = self.read_pkg_child_graph(graph.clone(), visited)?;

        if graph.name.is_empty() {
            graph.name = name;
            graph.version = version;
        }
        Ok(graph)
    }
}
