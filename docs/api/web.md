
# web {#web}

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

* `/pkg` 提供了获取当前项目依赖信息。
* `/env` 提供获取当前环境下的`node` 版本信息，包括：`npm`版本、`pnpm`版本、`yarn`版本。

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
* `/update` 更新指定的依赖到指定版本
* `/graph` 查询指定依赖的依赖关系图。

### `/env` {#web-http-env}

## `web socket` 服务

主要用于获取当前项目依赖的最新版本信息，依赖信息是异步请求，前端页面实现加载状态。在请求完成后发送消息给前端。
