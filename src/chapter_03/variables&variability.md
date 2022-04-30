# 变量与可变性

## 变量的声明

第一，声明变量使用`let`关键字

* 创建一个新项目：`cargo new variables`
* 进入项目目录下：`cd car*`
* 打开`sublime text3`：`subl .`

第二，默认情况下，变量是不可变的（Immutable）

第三，声明变量时，在变量前面加上`mut`，就可以使变量可变。

```rust
let mut x = 5;
println!("The value of x is {}", x);

x = 6; // 不可变的变量不可赋值
println!("The value of x is {}", x);
```

## 变量与常量

第一，常量（constant），常量在绑定值以后也是不可变的，但是它与不可变的变量有很多区别：

* 不可以使用`mut`，常量永远都是不可变的
* 声明常量必须`const`关键字，它的类型必须被标注
* 常量可以在任何作用域内进行声明，包括全局作用域
* 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值

第二，在程序运行期间，常量在其声明的作用域内一直有效。

第三，命名规范：Rust里常量使用全大写字母，每个单词之间用下划线分开，例如：

* MAX_POINTS
```rust
const MAX_POINTS: u32 = 100_000; // 增加下划线可增强可读性
println!("The const value is {}", MAX_POINTS);
```

## 隐藏（shadow）

第一，可以使用相同的名字声明新的变量，新的变量就会`shadow`之前声明的同名变量。

* 在后续的代码中这个变量名代表的就是新的变量

```rust
let y = 5; // 声明一个不可变的变量
// y = y + 1; // 报错，不可变的变量无法赋值
let y = y + 1; // shadow
let y = y * 2; // shaow

println!("The result of y is {}", y);
```

第二，`shadow`和把变量标记为`mut`是不一样的

* 如果不适用`let`关键字，那么重新给非`mut`得变量赋值会导致编译时错误
* 使用`let`声明的同名新变量，也是不可变的
* 使用`let`声明的同名新变量，它的类型可以与之前不同

```rust
let spaces = "    ";
let spaces = spaces.len();

println!("The length of spaces is {}", spaces);
```