use crate::manager::{PkgInfo, PkgLock};
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
/// 依赖关系图
/// 查询某个依赖的依赖关系图
///
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

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
    #[serde(default)]
    pub dependencies: HashMap<String, PkgDependence>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, PkgDependence>,
    #[serde(default)]
    pub dep_name: String,
    #[serde(default)]
    pub pkg_info: PkgInfo,
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
            packages: HashMap::new(),
            dep_name: Default::default(),
            pkg_info: PkgInfo::default(),
        }
    }
}

impl PkgLock for Pkg {
    fn new(dep_name: String, file_path: String) -> Pkg {
        println!("开始从{}读取依赖关系...", file_path);

        let mut pkg = Pkg::read_pkg(file_path).unwrap();
        println!("读取{}依赖关系完成", &dep_name);
        pkg.dep_name = dep_name;

        pkg
    }

    /// 读取某个依赖的依赖关系图
    fn read_pkg_graph(&mut self) -> Result<PkgInfo, Box<dyn Error>> {
        // 如果当前npm版本很低，则不支持查询
        // if self.lockfile_version < 2 {
        //     return Err("当前pnpm版本不支持查询依赖关系图".into());
        // }

        let dependence = if self.dependencies.contains_key(&self.dep_name) {
            self.dependencies.get(&self.dep_name).unwrap()
        } else {
            self.dev_dependencies.get(&self.dep_name).unwrap()
        };
        let key = format!("/{}@{}", self.dep_name, dependence.version);
        if !self.packages.contains_key(&key) {
            return Err(format!("当前路径未找到依赖：{}", self.dep_name).into());
        }
        // 记录依赖已被访问
        // self.visited.insert(key.clone(), self.pkg_info.clone());

        self.pkg_info = self.packages.get(&key).unwrap().clone();
        // 当前依赖名称设置为顶层路径的依赖名
        self.pkg_info.name = self.dep_name.clone();
        self.pkg_info.path = key.clone();
        self.pkg_info.version = dependence.version.clone();
        // 开始递归查找依赖关系图

        let relations = self
            .read_pkg_child_graph(self.pkg_info.clone(), vec![key])
            .unwrap();

        self.pkg_info.relations = relations;
        Ok(self.pkg_info.clone())
    }
}

impl Pkg {
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
        &mut self,
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
        &mut self,
        name: String,
        version: String,
        visited: Vec<String>,
    ) -> Result<PkgInfo, Box<dyn Error>> {
        let key = format!("/{}@{}", &name, &version);
        println!("开始递归读取依赖关系图,当前依赖：{:#?}", &key);
        let mut graph = PkgInfo::default();

        if self.packages.contains_key(&key) {
            println!("找到依赖：{}", &key);
            graph = self.packages.get(&key).unwrap().clone();
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
        }

        if graph.name.is_empty() {
            graph.name = name;
            graph.version = version;
        }
        Ok(graph)
    }
}
