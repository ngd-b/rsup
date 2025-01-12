# 开始使用 {#start}

在安装了[macos](../installer/macos) 或[windows](../installer/windows) 之后，可以在全局使用`rsup`工具了。

在命令行任意目录下都可以使用。执行目录后会自动读取当前目录下的`package.json`文件，获取其中的依赖信息，包括`dependencies`和`devDependencies`。同时启动一个`web`服务，并自动打开浏览器。

```sh
rsup
```

## 通过`--dir`指定项目目录 {#start-dir}

当在一个没有`package.json`目录下执行命令时，可以通过`--dir`指定项目目录。**必须是绝对路径**

```sh
rsup --dir /Users/admin/giteeCode/rsup/package.json
```

也可以不用指定到具体文件，为项目目录即可`/Users/admin/giteeCode/rsup/`。

启动`web`服务自动打开浏览器，交互功能查看[web功能](./web)

## 在项目目录下直接执行 {#start-in-project-dir}

在项目目录下，执行`rsup`命令，会自动读取`package.json`文件，并启动`web`服务。

```sh
rsup
```

启动`web`服务自动打开浏览器，交互功能查看[web功能](./web)
