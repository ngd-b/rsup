
# `Web` {#web}

`web`功能包用于创建一个web服务器，提供了前端交互访问的`http`接口、`web socket`、`web`静态资源服务。

在[开始](/guide/installer/macos)中创建了`rsup`的配置文件`config.toml`,其中包含了`web`服务的配置。

```toml
[web]
port = 8888
static_dir = "/opt/rsup/web"
```

配置端口号默认`8888`,配置静态资源目录,默认为`/opt/rsup/web`(这是`macos`,`windows`[系统查看](/guide/installer/windows))。

`web`服务提供的数据能力可以为自己所用，不使用默认提供的前端页面能力。可用于服务端、web端、桌面端。

## `http` 服务 {#web-http}

`http`请求接口前缀为`/api`,分功能模块，又分为：

* `/pkg/*` 提供了获取项目依赖相关的信息。
* `/env/*` 提供获取当前环境下的`node` 版本信息，包括：`npm`版本、`pnpm`版本、`yarn`版本。

`http`请求统一响应格式为：

```rust:line-numbers {1}
#[derive(Deserialize, Serialize)]
pub struct ResParams<T> {
    success: bool,
    msg: String,
    data: Option<T>,
}
impl<T> ResParams<T> {
    pub fn ok(data: T) -> ResParams<T> {
        ResParams {
            success: true,
            data: Some(data),
            msg: String::from("ok"),
        }
    }
    pub fn err(msg: String) -> ResParams<T> {
        ResParams {
            success: false,
            data: None,
            msg,
        }
    }
}
```

### `/pkg` {#web-http-pkg}

* `/get` 返回所有依赖信息。

    没有请求参数;

    响应`Response`返回当前项目所有的依赖最新版本信息。数据结构信息查看[`Pkg`数据结构定义](./pkg#pkg-pkg)

* `/update` 更新指定的依赖到指定版本

    升级指定的依赖到指定版本。

    请求参数结构体定义`Request`：

    ```rs
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct UpdateParams {
        pub name: String,
        pub version: String,
        pub is_dev: bool,
        // 是否是切换依赖类型
        // 开发依赖包、运行时依赖包
        pub is_change: Option<bool>,
    }
    ```

    响应`Response`为标准响应结构体。

* `/graph` 查询指定依赖的依赖关系图。

    请求参数结构体定义`Request`：

    ```rs
    #[derive(Deserialize, Serialize, Debug)]
    pub struct RelationGraphReq {
        pub name: String,
    }
    ```

    响应`Response`数据为当前依赖数据信息，包括根据`lock`文件解析出的关系数据。数据结构定义请查看[`PkgInfo`数据结构定义](./pkg#pkg-info)

* `/remove` 移除指定的依赖

    请求参数结构体定义`Request`：

    ```rs
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RemoveParams {
        pub name: String,
        pub is_dev: bool,
    }
    ```

    响应`Response`为标准响应结构体。

* `/quickInstall` 快速一键安装依赖

    请求参数结构体定义`Request`：

    ```rs
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct QuickInstallParams {
        pub manager_name: String,
        pub is_registry: bool,
        pub registry: Option<String>,
        pub params: Option<Vec<String>>,
    }
    ```

    响应`Response`为标准响应结构体。

* `/batchUpdate` 批量更新依赖

    请求参数结构体定义`Request`,批量更新分为`pathc/minor/major`版本更新，更新参数为单个`UpdateParams`结构体的集合`Vec<UpdateParams>`.

    响应参数结构体定义`Response`为标准响应结构体,响应数据为集合`Vec<String>`.

* `/reload` 重新加载依赖

    请求参数无.

    响应参数结构体定义`Response`为标准响应结构体.

### `/env` {#web-http-env}

* `/get` 返回当前环境`node`\ `pnpm` \ `npm` 等版本信息。

    没有请求参数;

    响应`Response`返回当前环境`node`\ `pnpm` \ `npm` 等版本信息。

    ```rs
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
    ```

## `web socket` 服务 {#web-ws}

主要用于获取当前项目依赖的最新版本信息，依赖信息是异步请求，前端页面实现加载状态。在请求完成后发送消息给前端。

本地服务启动后，访问`/ws` 可连接websocket服务。支持多个客户端连接，数据之间是共享的。
