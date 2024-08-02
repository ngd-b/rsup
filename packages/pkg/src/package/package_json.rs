use serde_derive::{Deserialize, Serialize};

use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

/// define the attributes of package.json
///
/// the `dependencies` and `devDependencies` are optional
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgJson {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
}

pub fn read_pkg_json<P: AsRef<Path>>(path: P) -> Result<PkgJson, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package = serde_json::from_reader(reader)?;
    Ok(package)
}
