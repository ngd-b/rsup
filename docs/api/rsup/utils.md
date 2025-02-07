
# `utils` {#utils}

`utils`提供全局通用的一些工具函数。

> [!IMPORTANT]
> 由于`rust-analyzer` 语法检测的问题，会提示`utils`下的方法未定义，所以修改`Cargo.toml`的名称`name = "rsup_utils"`

>[!NOTE]
> 其他功能包安装使用的是`rs_utils`,由于需要发布到`crates-io`。但是已经存在同样的名称，所以修改为`rsup_utils`

在使用`utils`时，安装依赖名称为`rsup_utils`. [访问crates.io](https://crates.io/crates/rsup_utils)

## `fs` {#uitls-fs}

`fs`提供了文件下载、文件解压的工具函数。

* `download_file(client: &Client, url: &str, output: &str) -> Result<(), Box<dyn Error>>` 下载文件`url`到指定路径`output`

* `decompress_file(url: &str, target_dir: &str) -> Result<(), Box<dyn Error>>`  解压指定文件`url`到指定路径`target_dir`

## `env` {#uitls-env}

`env`提供了环境变量的获取工具函数。提供了诸如`node`、`npm`、`pnpm`、`yarn`等环境变量的获取。

```rs:line-numbers
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
```
