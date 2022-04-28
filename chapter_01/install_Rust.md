# 安装Rust

## 如何自定义路径安装

&emsp;&emsp;首先定义路径，比如说，你需要将`.cargo`文件夹放在`D`盘下，那么你需要：

```textile
CARGO_HOME: D:\Rust\.cargo
```

同样地，将`.rustup`文件夹放在`D`盘下的命令是：

```textile
RUSTUP_HOME: D:\Rust\.rustup
```
Windows环境下，使用`rustup-init.exe`。

Linux or Mac环境下：

```textile
curl https://sh.rustup.rs -sSf | sh
```

## 更新与卸载Rust

更新Rust

---> `rustup update`

卸载Rust:

---> `rustup install uninstall`

## 安装验证

* `rustc --version`

---> 结果格式: rustc x.y.z (abcabcabc yyyy-mm-dd)

---> 会显示最新稳定版的：版本号、commit hash、commit 日期

## 本地文档

* 安装Rust的时候，还会在本地安装文档，可离线浏览
* 运行`rustup doc`可在浏览器打开本地文档

## 开发工具
* Visual Studio Code

---> Rust插件

* CLion（Intellij Idea系列）

---> Rust插件

* `Sublime Text`

---> 我使用的