use futures_util::future;
use rs_utils;
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
    // 是否是切换依赖类型
    // 开发依赖包、运行时依赖包
    pub is_change: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoveParams {
    pub name: String,
    pub is_dev: bool,
}

/// 一键快速安装依赖结构题

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickInstallParams {
    pub manager_name: String,
    pub is_registry: bool,
    pub registry: Option<String>,
    pub params: Option<Vec<String>>,
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

/// 一键安装依赖
///
///
pub async fn quick_install_dependencies(
    file_path: String,
    params: QuickInstallParams,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "will quick install dep in the path {} use {}",
        &file_path, &params.manager_name
    );

    // 项目所在目录
    let path = Path::new(&file_path);

    let dir_path = path.parent().unwrap().to_path_buf();

    let command_info = rs_utils::env::Env::new(&params.manager_name);
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
            return Err(format!("Not Found Env {}", &params.manager_name).into());
        }
    };

    let mut args = vec![];
    args.push("install".to_string());
    if params.is_registry {
        args.push("--registry".to_string());
        args.push(params.registry.unwrap().clone());
    }
    if let Some(params) = params.params {
        args.extend(params);
    }
    // 构建 npm install 命令
    let output = Command::new(npm_cmd)
        .args(args)
        .current_dir(&dir_path) // 设置执行命令的目录
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // .status()?; // 执行命令并等待结果
        .output()?;

    if output.status.success() {
        println!("Successfully installed");

        // 成功后需要更新全局的数据
        Ok(())
    } else {
        // 将错误信息发送给前端
        let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();
        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();

        let error_message = format!("Failed to install",);
        println!("stderr_str: {}", stderr_str);
        println!("stdout_str: {}", stdout_str);

        Err(error_message.into())
    }
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

    let command_info = rs_utils::env::Env::new(&manager_name);
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
    // 构建命令参数
    let mut args = vec![];
    args.push("install");
    args.push(&name);
    if params.is_dev {
        args.push("-D");
    }
    // 构建 npm install 命令
    let output = Command::new(npm_cmd)
        .args(args)
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

/// 删除某个npm依赖包
///
/// 来自于web服务调用
pub async fn remove_dependencies(
    file_path: String,
    params: RemoveParams,
    manager_name: String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!(
        "will remove dep info {} remove {} in the path {}",
        &manager_name, &params.name, &file_path
    );

    // 项目所在目录
    let path = Path::new(&file_path);

    let dir_path = path.parent().unwrap().to_path_buf();

    let command_info = rs_utils::env::Env::new(&manager_name);
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

    // 构建 remove 命令
    let output = Command::new(npm_cmd)
        .arg("remove")
        .arg(&params.name)
        .current_dir(&dir_path) // 设置执行命令的目录
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // .status()?; // 执行命令并等待结果
        .output()?;

    if output.status.success() {
        println!("Successfully removed {}", &params.name);

        // 成功后需要更新全局的数据
        Ok(None)
    } else {
        // 将错误信息发送给前端
        let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();
        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();

        let error_message = format!("Failed to remove {}", &params.name,);
        println!("stderr_str: {}", stderr_str);
        println!("stdout_str: {}", stdout_str);

        Err(error_message.into())
    }
}

/// 批量更新依赖
///
pub async fn batch_update_dependencies(
    file_path: String,
    params: Vec<UpdateParams>,
    manager_name: String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!(
        "will update all deps info {} in the path {}",
        &manager_name, &file_path
    );
    // 记录哪些依赖包更新成功；哪些依赖包更新失败

    // 并发执行更新
    let futures = params.into_iter().map(|param| {
        let file_path = file_path.clone();
        let manager_name = manager_name.clone();

        let param = param.clone();

        tokio::spawn(async move {
            match update_dependencies(file_path, param.clone(), manager_name).await {
                Ok(_) => {
                    println!("update dep {} success", &param.name);
                    Some(param.name)
                }
                Err(e) => {
                    println!("update dep {} failed {}", &param.name, e);
                    None
                }
            }
        })
    });

    let success_deps = future::join_all(futures).await;

    let mut deps = vec![];
    for dep in success_deps {
        if let Ok(Some(name)) = dep {
            deps.push(name)
        }
    }

    Ok(deps)
}
