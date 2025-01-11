# Macos 安装 {#macos}

## 脚本安装 {#macos-install}

`rsup`提供了安装脚本，通过安装脚本、执行脚本安装来`rsup`。

```sh
curl -fsSL https://raw.githubusercontent.com/ngd-b/rsup/main/install.sh | sh
```

脚本安装不需要手动处理任何东西，等待安装完成后即可[开始使用](../start/base)

## 手动安装 {#macos-manual}

如果使用脚本安装有问题，可以下载安装程序，然后执行安装程序来安装`rsup`。[下载地址](https://github.com/ngd-b/rsup/releases/download/latest/rsup-installer-macos-latest.tar.gz)

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

有版本信息的打印输出，说明我们的`rsup`工具就安装好了 [开始使用](../star/base)

### 全手动安装 {#macos-manual-full}

下载文件主要包含`rsup`和`rsup-web`两个文件，有时在命令行中下载缓慢，但是两个文件其实只有几MB，在浏览器中下载可能更快，所以我们可以直接下载资源，然后放到指定目录下即可。

#### 创建`/opt/rsup`目录

```sh
mkdir /opt/rsup
```

#### 创建配置文件`config.toml`

```sh
cd /opt/rsup/
# 
vi config.toml
```

直接复制一下文件内容：

```toml
name = "rsup"
version = "0.3.0"
dir = "/opt/rsup"

[web]
port = 8888
static_dir = "/opt/rsup/web"

[pkg]
npm_registry = "https://registry.npmmirror.com"
```

不需要做什么修改，直接保存。

#### 下载两个资源`rsup` 和`rsup-web`

[rsup下载资源地址](https://github.com/ngd-b/rsup/releases/download/latest/rsup-macos-latest.tar.gz)

[rsup-web下载资源地址](https://github.com/ngd-b/rsup-web/releases/download/latest/rsup-web.tar.gz)

两个资源包都是压缩包，都是发布在github上的，下载地址不会发生更改。如果功能有更新可以直接下载替换。

文件下载后解压，直接将解压文件复制到创建的目录`/opt/rsup`下即可。

```sh
# 这里解压的rsup-web文件 我们重命名为 web
mv rsup-web /opt/rsup/web
# 解压的rsup执行文件
mv rsup /opt/rsup/
```

#### 配置添加到系统环境变量

```sh
echo 'export PATH=$PATH:/opt/rsup' >> ~/.zshrc
# 刷新环境变量
source ~/.zshrc
```

通过`rsup -V`查看是否安装成功。

```sh
rsup -V
```

有版本信息的打印输出，说明我们的`rsup`工具就安装好了 [开始使用](../start/base)
