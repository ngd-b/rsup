use serde_derive::{Deserialize, Serialize};
/// 依赖关系图
/// 查询某个依赖的依赖关系图
///
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pkg {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    #[serde(rename = "lockfileVersion")]
    pub lockfile_version: i32,
    #[serde(default)]
    pub packages: HashMap<String, PkgInfo>,
    #[serde(default)]
    pub dep_name: String,
    #[serde(default)]
    pub pkg_info: PkgInfo,
    // 记录是否访问过，防止循环依赖
    #[serde(default)]
    pub visited: HashMap<String, PkgInfo>,
}

impl Default for Pkg {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Default::default(),
            lockfile_version: Default::default(),
            packages: HashMap::new(),
            dep_name: Default::default(),
            pkg_info: PkgInfo::default(),
            visited: HashMap::new(),
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
            dependencies: HashMap::new(),
            is_peer: false,
            is_loop: false,
            relations: vec![],
            path: String::new(),
        }
    }
}

impl Pkg {
    pub fn new(dep_name: String, file_path: String) -> Pkg {
        println!("开始从{}读取依赖关系...", file_path);

        let mut pkg = Pkg::read_pkg(file_path).unwrap();
        println!("读取{}依赖关系完成", &dep_name);
        pkg.dep_name = dep_name;

        pkg
    }
    // 读取的package.json文件
    fn read_pkg(file_path: String) -> Result<Pkg, Box<dyn Error>> {
        // 项目所在目录
        let path = Path::new(&file_path);

        let dir = path.parent().unwrap();
        // 读取package.json文件
        let pkg_path = dir.join("package-lock.json");

        if !pkg_path.exists() {
            return Err("package-lock.json文件不存在".into());
        }

        let file = File::open(pkg_path)?;
        let reader = BufReader::new(file);
        let package = serde_json::from_reader(reader)?;

        Ok(package)
    }
    /// 读取某个依赖的依赖关系图
    pub fn read_pkg_graph(&mut self) -> Result<(), Box<dyn Error>> {
        // 如果当前npm版本很低，则不支持查询
        if self.lockfile_version < 2 {
            return Err("当前npm版本不支持查询依赖关系图".into());
        }
        // 嵌套路径
        let prefix = [self.dep_name.clone()].to_vec();
        let key = format!("{}/{}", "node_modules", prefix.join("/node_modules/"));
        if !self.packages.contains_key(&key) {
            return Err(format!("当前路径未找到依赖：{}", self.dep_name).into());
        }
        // 记录依赖已被访问
        // self.visited.insert(key.clone(), self.pkg_info.clone());

        self.pkg_info = self.packages.get(&key).unwrap().clone();
        // 当前依赖名称设置为顶层路径的依赖名
        self.pkg_info.name = self.dep_name.clone();
        self.pkg_info.path = key;
        // 开始递归查找依赖关系图
        let key = format!("{}@{}", self.pkg_info.name, self.pkg_info.version);
        let relations = self
            .read_pkg_child_graph(self.pkg_info.clone(), prefix, vec![key])
            .unwrap();

        // 不能递归查找依赖关系图，应该采用bfs的方式查找依赖关系图
        // let mut queue = VecDeque::new();
        // queue.push_back(&mut relations);
        // // 记录已访问的依赖
        // let mut visited = HashSet::new();
        // // 记录已访问的依赖，防止循环依赖
        // visited.insert(format!("{}@{}", self.pkg_info.name, self.pkg_info.version));
        // while let Some(nodes) = queue.pop_front() {
        //     // bfs
        //     for node in nodes {
        //         let key = format!("{}@{}", node.name, node.version);
        //         if visited.contains(&key) {
        //             node.is_loop = true;
        //             continue;
        //         }
        //         visited.insert(key.clone());
        //         // 将子依赖加入队列
        //         queue.push_back(&mut node.relations);
        //     }
        // }

        self.pkg_info.relations = relations;
        Ok(())
    }

    fn read_pkg_child_graph(
        &mut self,
        parent: PkgInfo,
        prefix: Vec<String>,
        visited: Vec<String>,
    ) -> Result<Vec<PkgInfo>, Box<dyn Error>> {
        let mut relations = Vec::new();

        let mut process_dependencies = |dependencies: &HashMap<String, String>, is_peer: bool| {
            for (child_name, _) in dependencies.iter() {
                let mut prefix = prefix.clone();
                prefix.push(child_name.to_string());
                // 递归查找依赖关系图
                let mut child = self
                    .read_pkg_graph_recursively(prefix, visited.clone())
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
        prefix: Vec<String>,
        visited: Vec<String>,
    ) -> Result<PkgInfo, Box<dyn Error>> {
        println!(
            "开始递归读取依赖关系图,当前依赖：{:#?}",
            &prefix.last().unwrap()
        );
        let mut graph = PkgInfo::default();

        let mut keys = prefix.clone();
        while keys.len() > 0 {
            let key: String = format!("{}/{}", "node_modules", keys.join("/node_modules/"));

            println!("正在查找依赖,依赖路径：{}", key);
            if self.packages.contains_key(&key) {
                println!("找到依赖：{}", &key);
                graph = self.packages.get(&key).unwrap().clone();
                graph.name = keys.last().unwrap().to_string();
                graph.path = key.clone();
                // 判断是否存在循环依赖
                // if self.visited.contains_key(&key) {
                //     graph = self.visited.get(&key).unwrap().clone();
                //     // graph.is_loop = true;
                //     // 记录循环依赖
                //     println!("存在循环依赖：{}", &key);
                //     break;
                // }
                // // 缓存已访问的依赖
                // self.visited.insert(key.clone(), graph.clone());
                let visited_name = format!("{}@{}", graph.name, graph.version);
                if visited.contains(&visited_name) {
                    graph.is_loop = true;
                    println!("存在循环依赖：{}", &key);
                    break;
                }
                let mut visited = visited.clone();
                visited.push(visited_name);
                // 递归处理依赖关系图
                graph.relations =
                    self.read_pkg_child_graph(graph.clone(), prefix.clone(), visited)?;

                break;
            }
            // 删除倒数第二个元素
            if keys.len() <= 1 {
                break;
            }
            keys.remove(keys.len() - 2);
        }
        // 没有查到
        if graph.name.is_empty() {
            graph.name = prefix.last().unwrap().to_string();
        }
        Ok(graph)
    }
}
