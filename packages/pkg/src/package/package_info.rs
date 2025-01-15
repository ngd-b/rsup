use regex::Regex;
use reqwest::Client;

use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

/// define the attributes of package info
///
/// the main attributes are `name`, `description`, `dist-tags` and `versions`.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgInfo {
    pub name: String,
    pub readme: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub versions: HashMap<String, VersionInfo>,
    #[serde(default)]
    pub is_dev: bool,
    #[serde(default)]
    pub is_finish: bool,
    #[serde(default)]
    pub is_del: bool,
}
impl Default for PkgInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: None,
            readme: None,
            description: None,
            homepage: None,
            keywords: None,
            license: None,
            dist_tags: DistTags {
                latest: String::new(),
            },
            versions: HashMap::new(),
            is_dev: false,
            is_finish: false,
            is_del: false,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistTags {
    pub latest: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dist {
    pub shasum: Option<String>,
    pub size: Option<usize>,
    pub tarball: Option<String>,
    pub integrity: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memeber {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Author {
    Memeber(Memeber),
    String(String),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionInfo {
    pub name: String,
    pub version: Option<String>,
    pub homepage: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub author: Option<Author>,
    pub maintainers: Option<Vec<Memeber>>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<HashMap<String, String>>,
    pub dist: Option<Dist>,
}

/// fetch the package info from registry
pub async fn fetch_pkg_info(
    client: &Client,
    pkg_name: &str,
    registry: &str,
) -> Result<PkgInfo, Box<dyn std::error::Error>> {
    // let url = format!("https://registry.npmjs.org/{}", pkg_name);
    // let url = format!("https://registry.npmmirror.com/{}", pkg_name);
    // let config = Config::get_config();
    let url = format!("{}/{}", registry, pkg_name);
    println!("Fetching info for: {}", url);

    let res = client
        .get(&url)
        // .timeout(Duration::from_millis(30))
        .send()
        .await?;

    println!("Received response with status: {}", res.status());
    if res.status().is_success() {
        let body = res.text().await?;

        println!("Response body length: {}", body.len());
        let info: PkgInfo = serde_json::from_str(&body)?;

        Ok(info)
    } else {
        let error_message = format!("Request failed with status code: {}", res.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        )))
    }
}

/// compare the version and return a new map of last versions
pub fn compare_version(
    current_v: &str,
    latest_v: &str,
    all_v: HashMap<String, VersionInfo>,
) -> HashMap<String, VersionInfo> {
    let clear_current_v = clear_version(current_v);
    let c_v = Version::parse(&clear_current_v).unwrap();
    let l_v = Version::parse(&latest_v).unwrap();

    let mut vs: Vec<Version> = all_v
        .keys()
        .filter_map(|k| Version::parse(k).ok())
        .filter(|v| *v > c_v && *v <= l_v)
        .collect();

    // 不需要预发布版本
    vs.retain(|v| v.pre.is_empty());
    // vs.sort();
    // 版本从高到低排序
    // vs.sort_by(|a, b| a.cmp(b));

    let mut res: HashMap<String, VersionInfo> = HashMap::new();
    for v in vs {
        if let Some(info) = all_v.get(&v.to_string()).cloned() {
            res.insert(v.to_string(), info);
        }
    }
    res
}

/// 清除版本号中的前缀
fn clear_version(v: &str) -> String {
    let re = Regex::new(r"^[^\d]*(\d+\.\d+\.\d+).*").unwrap();
    re.captures(v)
        .and_then(|caps| caps.get(1))
        .map_or(v.to_string(), |m| m.as_str().to_string())
}
