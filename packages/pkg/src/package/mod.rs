use regex::Regex;
use reqwest::Client;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

/// define the attributes of package.json
///
/// the `dependencies` and `devDependencies` are optional
///
#[derive(Debug, Serialize, Deserialize)]
pub struct PkgJson {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
}

/// define the attributes of package info
///
/// the main attributes are `name`, `description`, `dist-tags` and `versions`.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct PkgInfo {
    pub name: String,
    pub description: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub versions: HashMap<String, Value>,
    pub homepage: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DistTags {
    pub latest: String,
}

/// read package.json from path
///
/// ```
/// let pkg_file_path = Path::new(&args.dir).join("package.json");
///
/// read_pkg_json(&pkg_file_path)
/// ```
///
pub fn read_pkg_json<P: AsRef<Path>>(path: P) -> Result<PkgJson, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package = serde_json::from_reader(reader)?;
    Ok(package)
}

/// fetch package info from registry.npmjs.org or registry.npmmirror.com
///
/// ```
/// let client: Client = Client::new();
///
/// let name = "vue";
/// fetch_pkg_info(&client, name).await
/// ```
///
pub async fn fetch_pkg_info(
    client: &Client,
    pkg_name: &str,
) -> Result<PkgInfo, Box<dyn std::error::Error>> {
    // let url = format!("https://registry.npmjs.org/{}", pkg_name);
    let url = format!("https://registry.npmmirror.com/{}", pkg_name);
    println!("Fetching info for: {}", url);
    let res = client.get(&url).send().await?;

    if res.status().is_success() {
        let info = res.json().await?;
        Ok(info)
    } else {
        Err(format!("Failed to fetch package info: HTTP {}", res.status()).into())
    }
}

/// compare version
///
pub fn compare_version(
    current_v: &str,
    latest_v: &str,
    all_v: &HashMap<String, Value>,
) -> Vec<String> {
    let c_v = Version::parse(&clear_version(current_v)).unwrap();
    let l_v = Version::parse(latest_v).unwrap();

    let mut vs: Vec<Version> = all_v
        .keys()
        .filter_map(|k| Version::parse(k).ok())
        .filter(|v| *v > c_v && *v <= l_v)
        .collect();

    vs.sort();
    vs.into_iter().map(|v| v.to_string()).collect()
}
fn clear_version(v: &str) -> String {
    let re = Regex::new(r"^[^\d]*(\d+\.\d+\.\d+).*").unwrap();
    re.captures(v)
        .and_then(|caps| caps.get(1))
        .map_or(v.to_string(), |m| m.as_str().to_string())
}
