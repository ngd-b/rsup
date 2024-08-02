use std::collections::HashMap;

pub mod package_info;
pub mod package_json;

use package_info::PkgInfo;

#[derive(Debug, Clone)]
pub struct Pkg {
    pub name: String,
    pub version: String,
    pub description: String,
    pub scripts: HashMap<String, String>,
    pub dependencies: HashMap<String, PkgInfo>,
    pub dev_dependencies: HashMap<String, PkgInfo>,
}

impl Pkg {
    pub fn new() -> Self {
        Pkg {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            scripts: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        }
    }
}
