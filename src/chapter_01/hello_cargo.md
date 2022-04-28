# Cargo

## 介绍

* `Cargo`是Rust的构建系统和包管理工具
---> 构建代码，下载依赖库、构建这些库...

* 安装`Rust`的时候会安装`Cargo`
---> `cargo --version`

## 使用Cargo创建项目

* 创建项目: `cargo new hello_cargo`

---> 项目名称: `hello_cargo`

---> 创建一个新的目录: `hello_cargo`

------> Cargo.toml

------> src 目录

---------> main.rs

------> 初始化了一个新的Git仓库，`.gitignore`

---------> 可以使用其它的VCS或不使用VCS： cargo new的时候使用--VCS这个flag


* 查看是否创建成功： `mkdir hello_cargo`

* 进入项目文件夹： `cd hello_cargo`

* 命令行打开工具
---> Vscode: `code .`

---> sublime text3： `subl .`

## Cargo.toml

* TOML（Tom's Obvious, Minimal Language）格式，是Cargo的配置格式

* 【package】，是一个区域标题，表示下方内容是用来配置包（package）的

---> name: 项目名

---> version: 项目版本

---> authors: 项目作者

---> edition: 使用的Rust版本

* 【dependencies】，另一个区域的开始，他会列出项目的依赖项

* 在Rust里面，代码的包称作crate。

## src/main.rs

* cargo生成的main.rs在src目录下

* Cargo.toml在项目的顶层目录下

* 源代码都应该在src目录下

* 顶层目录可以放置：README、许可信息、配置文件和其它与程序源码无关的文件

* 如果创建项目时没有使用cargo，也可以把项目转化为cargo使用

## 构建Cargo项目`cargo build`

* cargo build

---> 创建可执行文件： target/debug/hello_cargo.exe

---> 运行可执行文件：./target/debug/hello_cargo.exe

* 第一次运行cargo build会在顶层目录生成cargo.lock文件

---> 该文件负责追踪项目依赖的精确版本

---> 不需要手动修改该文件

## 构建和运行Cargo项目`cargo run`

* cargo run, 编译代码 + 执行结果

---> 如果之前编译成功过，并且源码没有改变，那么就会直接运行二进制文件

---> 如果修改过代码，那么会重新编译一次文件

## cargo check

* cargo check, 检查代码，确保能通过编译，但是不产生任何可执行文件

* cargo check要比cargo build快得多

---> 编写代码的时候可以**连续反复**的使用`cargo check`检查代码，提高效率

## 为发布构建

* cargo build --release

---> 编译时会进行优化

------> 代码会运行的更快，但是编译时间会更长

---> 会在target/release而不是target/debug生成可执行文件

* 两种配置

---> 一个开发

---> 一个正式发布

> 尽量使用Cargo，功能强大，简化操作步骤