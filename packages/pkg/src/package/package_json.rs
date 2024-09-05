use serde_derive::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::Path,
    process::{Command, Stdio},
};

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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateParams {
    pub name: String,
    pub version: String,
    pub is_dev: bool,
}

/// read package.json file from the path and parse it into PkgJson struct
///
pub fn read_pkg_json<P: AsRef<Path>>(
    path: P,
) -> Result<PkgJson, Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package = serde_json::from_reader(reader)?;
    Ok(package)
}

/// 更新某个npm依赖包
///
/// 来自于web服务调用
pub async fn update_dependencies(
    file_path: String,
    params: UpdateParams,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!(
        "will update dep info :{} in the path {}",
        &params.name, &file_path
    );

    // 项目所在目录
    let path = Path::new(&file_path);

    let dir_path = path.parent().unwrap().to_path_buf();

    let name = format!("{}@{}", params.name, params.version);

    // 构建 npm install 命令
    let output = Command::new("npm")
        .arg("install")
        .arg(&name)
        .current_dir(&dir_path) // 设置执行命令的目录
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // .status()?; // 执行命令并等待结果
        .output()?;

    if output.status.success() {
        println!("Successfully installed {}", &name);

        // 成功后需要更新全局的数据
        Ok(None)
    } else {
        println!("Failed to install {}", &name);
        // Err(format!("Failed to install {}", &name).into())

        // 将错误信息发送给前端
        let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(stderr_str.into())
    }
}
