# Windows 安装 {#windows}

## 脚本安装 {#windows-install}

## 手动安装 {#windows-manual}

手动安装针对不能使用安装脚本的用户。也可以了解到安装的过程以及生成的文件。

### 半手动安装 {#macos-manual-half}

如果使用脚本安装有问题，可以下载安装程序，然后执行安装程序来安装`rsup`。[下载地址](https://github.com/ngd-b/rsup-intaller/releases/download/latest/rsup-installer-windows-latest.zip)

下载完文件是一个压缩包，完成解压。可以得到一个`installer`安装程序。

![alt text](/assets/macos-installer.png)

使用管理员权限打开终端，执行解压的可执行程序：

```sh
sudo /Users/admin/Downloads/installer 
```

![alt text](/assets/macos-installer-process.png)

开始下载资源，默认下载安装目录为`/opt/rsup/`,目录包含三个文件：

* `config` 配置文件
* `rsup` 可执行文件
* `web` web服务静态资源

下载完成后，会自动将命令添加到环境变量中，默认添加到`~/.zshrc`文件中。通过`source ~/.zshrc` 刷新环境变量。

通过`rsup -V`查看是否安装成功。

```sh
rsup -V
```

有版本信息的打印输出，说明我们的`rsup`工具就安装好了 [开始使用](../start/base)

### 全手动安装 {#windows-manual-full}

有时在命令行中下载缓慢，但是两个文件其实只有几MB，在浏览器中下载可能更快，所以我们可以直接下载资源，然后放到指定目录下即可。

下载文件主要包含`rsup`和`rsup-web`两个文件。

#### 创建`C:\Program Files\rsup`目录 {#windows-manual-full-create-dir}

> [!IMPORTANT]
>存放位置暂时不支持自定义其他位置存放。

#### 创建配置文件`config.toml` {#windows-manual-full-create-config}

直接复制一下文件内容：

```toml
name = "rsup"
version = "0.3.0"
dir = 'C:\Program Files\rsup'

[web]
port = 8888
static_dir = 'C:\Program Files\rsup/web'

[pkg]
npm_registry = "https://registry.npmmirror.com"
```

不需要做什么修改，直接保存。

#### 下载两个资源`rsup` 和`rsup-web` {#windows-manual-full-download-resources}

[rsup下载资源地址](https://github.com/ngd-b/rsup/releases/download/latest/rsup-windows-latest.zip)

[rsup-web下载资源地址](https://github.com/ngd-b/rsup-web/releases/download/latest/rsup-web.tar.gz)

两个资源包都是压缩包，都是发布在github上的，下载地址不会发生更改。如果功能有更新可以直接下载替换。

文件下载后解压，直接讲解压文件复制到创建的目录`C:\Program Files\rsup`下即可。

解压后的文件名称为 `rsup-web` 修改为`web`,安装成功后的目录是这样的

![image alt](/assets/windows-intaller-dir.png)

#### 配置添加到系统环境变量 {#windows-config-env}

手动配置环境变量，在系统变量`Path`中添加`C:\Program Files\rsup`即可。

![image alt](/assets/windows-config-env.png)

通过`rsup -V`查看是否安装成功。

```sh
rsup -V
```

有版本信息的打印输出，说明我们的`rsup`工具就安装好了 [开始使用](../start/base)
