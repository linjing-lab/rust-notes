# 猜数游戏: 一次猜测

本章将会学会：
* let、match等方法
* 相关的函数
* 外部的crate
* ...

## 目标

&emsp;&emsp;生成一个1到100间的随机数，提示玩家输入一个猜测，猜完之后，程序会提示猜测是太小了还是太大了。如果猜测正确，那么会打印出一个庆祝信息，程序退出。

## 写代码

创建新的项目：
* cargo new guess_once

进入到目录中：
* cd g*

使用sublime text打开
* subl .

## 小结

Rust中所有变量都是默认不可变的（immutable）

```rust
let foo = 1;
let bar = foo; // immutable，foo变量的值绑定到了bar变量上
foo = 2; // 会报错，rust默认所有变量都是不可变的
```

String::new()中的new是String类型的关联函数，创建一个空白的字符串

```rust
let mut guess = String::new(); // 将空字符串被绑定到了guess的值上
```

&引用默认不可变

```rust
use std::io; // 函数体外部使用
io::stdin().read_line(&mut guess).expect("无法读取行"); // & mut表示 guest输入与let mut guess指向同一片地址，而且可变
```

输出信息: io::Result Ok, Err，如果Err，会出现expect中的信息

```rust
println!("你猜测的数是: {}", guess); // {}是一个占位符
```