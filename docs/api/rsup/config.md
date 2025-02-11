
# `config` {#config}

`config`管理`rsup`的配置文件`config.toml`,可以生成、读取、修改配置文件。

`config`提供的配置读取、写入函数公用。在安装器执行时需要调用，所以发布到`crates-io`，包名为`rsup_config`.

在使用`config`时，安装依赖名称为`rsup_config`. [访问crates.io](https://crates.io/crates/rsup_config)

```rs:line-numbers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub dir: String,
    // web 服务配置
    pub web: WebConfig,
    // 包管理配置
    pub pkg: PkgConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebConfig {
    pub port: u16,
    pub static_dir: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgConfig {
    pub npm_registry: String,
}
```

为了方便在各个子包中访问配置文件数据并且避免重复读取文件，`config`提供了懒加载配置，确保了并发多线程下的读写安全。

```rs:line-numbers
pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    // 这里调用初始化
    let config = Config::read_config().unwrap();

    RwLock::new(config)
});
```
