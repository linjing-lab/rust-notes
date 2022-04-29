# 第一个.rs文件

## Shell常用指令

检查或创建项目文件夹下：

* `mkdir hello_world` ---> 确保项目文件夹存在，如果不存在就创建一个。

进入项目文件夹：

* `cd h*`

## Rust程序解剖

定义函数: `fn main() {}`

* 没有参数，没有返回

main 函数： 它是每个Rust可执行程序最先运行的代码

打印文本： `println!("Hello World!")`;

* Rust的缩进是4个空格而不是tab

* println!是一个Rust macro（宏）---> 如果是函数的话，就没有!

* "Hello World"是字符串，它是println!的参数

* 这行代码以;结尾

## 编译与运行

运行Rust程序之前必须先编译，命令为：rustc 源文件名

* rustc main.rs

编译成功后，会生成一个二进制文件

* 在windows上还会生成一个.pdb文件，里面包含调试信息。

Rust是`ahead-of-time`编译的语言

* 可以先编译程序，然后把可执行文件交给别人运行（无需安装Rust）
  
  `rustc`只适合简单的Rust程序

* 当程序比较大，文件比较多的时候，需要用到`cargo`

## 程序实例

[hello_world.rs](./hello_world/hello_world.rs)