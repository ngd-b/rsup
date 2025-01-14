use serde_derive::{Deserialize, Serialize};
use utils;

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
    manager_name: String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!(
        "will update dep info {} install {} in the path {}",
        &manager_name, &params.name, &file_path
    );

    // 项目所在目录
    let path = Path::new(&file_path);

    let dir_path = path.parent().unwrap().to_path_buf();

    let name = format!("{}@{}", params.name, params.version);

    let command_info = utils::rs_env::Env::new(&manager_name);
    let npm_cmd = match command_info {
        Some(env) => {
            // 判断系统，如果是windows，则使用npm.cmd
            if cfg!(windows) && env.is_cmd {
                format!("{}.cmd", env.name)
            } else {
                env.name
            }
        }
        None => {
            return Err(format!("Not Found Env {}", manager_name).into());
        }
    };

    // 构建 npm install 命令
    let output = Command::new(npm_cmd)
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
        // 将错误信息发送给前端
        let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();
        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();

        let error_message = format!("Failed to install {}", &name,);
        println!("stderr_str: {}", stderr_str);
        println!("stdout_str: {}", stdout_str);

        Err(error_message.into())
    }
}
