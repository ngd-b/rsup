
# `pkg` {#pkg}

`pkg` 包用于解析前端依赖文件，包括`package.json` \ `package-lock.json` 。对于不同依赖管理器，则对应不同的配置文件。比如`pnpm-lock.yaml` \ `yarn.lock`.

在[开始](/guide/installer/macos)中创建了`rsup`的配置文件`config.toml`,其中包含了`pkg`服务的配置。

```toml
[pkg]
npm_registry = "https://registry.npmmirror.com"
```

设置依赖源地址，默认是`npmmirror`源，可以设置为其他源地址。可以查看[如何修改配置](/guide/start/command)

`pkg`接收一个命令行参数`--dir`,用于指定项目目录，默认是当前目录。

```rs:line-numbers
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "The path to the package.json file"
    )]
    pub dir: String,
}
```

## `PkgJson` {#pkg-json}

`pkg`执行方法`fn run(args: Args, package: Package) {...}`读取指定目录下的`package.json`文件并解析为`PkgJson`数据结构

```rs:line-numbers
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
```

需要读取`dependencies`和`dev_dependencies`的依赖项，读取依赖的最新版本信息，最终生成我们想要的结构`Pkg`

## `Pkg` {#pkg-pkg}

当前项目的依赖信息,和`PkgJson`区别在于转换并增加了了一些字段，并且读取了`dependencies`和`dev_dependencies`中依赖的最新版本信息。

```rs:line-numbers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pkg {
    pub path: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub scripts: HashMap<String, String>,
    // 当前项目的管理工具
    pub manager_name: Option<String>,
    pub dependencies: HashMap<String, PkgInfo>,
    pub dev_dependencies: HashMap<String, PkgInfo>,
}
```

## `PkgInfo` {#pkg-info}

```rs:line-numbers
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
```

### `DistTags` {#pkg-dist-tags}

```rs:line-numbers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistTags {
    pub latest: String,
}
```

### `VersionInfo` {#pkg-version}

依赖版本信息。

```rs:line-numbers
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
```

#### `Author` {#pkg-info-author  }

```rs:line-numbers
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Author {
    Memeber(Memeber),
    String(String),
}
```

#### `Memeber` {#pkg-info-member  }

```rs:line-numbers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memeber {
    pub name: String,
    pub email: Option<String>,
}
```

#### `Dist` {#pkg-info-dist  }

```rs:line-numbers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dist {
    pub shasum: Option<String>,
    pub size: Option<usize>,
    pub tarball: Option<String>,
    pub integrity: Option<String>,
}
```
