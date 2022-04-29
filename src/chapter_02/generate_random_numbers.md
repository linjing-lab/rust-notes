# 猜数游戏: 生成神秘数字

## rand

&emsp;&emsp;Rust没有自带的生成随机数的功能，但是crate中包含了名为`rand`的包。可以在【dependencies】下方加入：
* rand="0.3.14" 或者 rand="^0.3.14"

> rand="^0.3.14"表示任何与`0.3.14`版本公共API兼容的版本都可以

## Cargo.lock

* 首次运行cargo build时，系统自动创建的
* 再次build时，Cargo.lock是否存在，存在则使用其中的依赖
* 如果想在当前版本下，更新这个版本（cargo update，会忽略Cargo.lock中的内容，并且根据依赖项重新写入Cargo.lock。）
* 如果版本依赖写的是`0.3.14`，那么只会升级到`0.3.x`的最新版本，不会升级到`0.4.x`版本。

## Cargo.toml

* 更改版本`0.7.0`
* 命令行执行`cargo build`

## 小结

函数体外部：

```rust
use rand::Rng; // trait or interface
```

`pub fn thread_rng() -> ThreadRng`：

`ThreadRng`是一个随机数生成器，位于本地线程空间，通过操作系统获得随机数的种子。

```rust
let secret_number = rand::thread_rng();
```