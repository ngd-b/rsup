use std::collections::HashMap;

pub mod package_info;
pub mod package_json;

use package_info::PkgInfo;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pkg {
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
            name: None,
            version: None,
            description: None,
            scripts: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        }
    }
}
