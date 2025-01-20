use std::process::Command;

use serde_derive::{Deserialize, Serialize};

/// 环境变量
/// 包括名称、版本、安装路径
#[derive(Deserialize, Serialize, Clone)]
pub struct Env {
    /// 命令名称
    pub name: String,
    /// 命令版本
    pub version: String,
    /// 命令路径
    pub path: String,
    /// 命令是否是批处理操作
    pub is_cmd: bool,
}

/// 定义了当前需要使用的系统环境变量
///
/// 包括node、npm、pnpm、yarn
///
#[derive(Deserialize, Serialize)]
pub struct EnvVec {
    /// node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Env>,
    /// npm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npm: Option<Env>,
    /// pnpm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnpm: Option<Env>,
    /// yarn
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn: Option<Env>,
}

impl Env {
    /// 创建一个环境变量
    ///
    /// # Arguments
    /// * name 命令名称
    ///
    /// # Returns
    /// * `Option<Env>`
    ///
    ///     如果命令不存在，返回`None`
    ///
    ///     如果命令存在，返回`Some(Env)`
    ///
    ///
    pub fn new(name: &str) -> Option<Self> {
        let mut env = Env {
            name: String::from(""),
            version: String::from(""),
            path: String::from(""),
            is_cmd: false,
        };

        match name {
            "node" => env.is_cmd = false,
            _ => env.is_cmd = true,
        }
        env.name = name.to_string();

        let env = get_command_info(env);

        if env.path.is_empty() {
            None
        } else {
            Some(env)
        }
    }
}

///
/// 获取命令信息
///
/// # Arguments
/// * command 命令名称
///
/// # Returns
/// * `Env`
///
///     返回命令信息
///
pub fn get_command_info(mut command: Env) -> Env {
    let which_cmd = if cfg!(windows) { "where" } else { "which" };
    // 路径
    let path = Command::new(which_cmd)
        .arg(&command.name)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        });

    let command_sys_cmd = if cfg!(windows) && command.is_cmd {
        format!("{}.cmd", &command.name)
    } else {
        command.name.clone()
    };
    command.path = path.unwrap_or_default();
    let version = Command::new(command_sys_cmd)
        .arg("-v")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        });
    command.version = version.unwrap_or_default();
    command
}
