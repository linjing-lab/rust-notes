# 函数

第一，声明函数使用`fn`关键字。

第二，针对函数和变量名，Rust使用snake case命名规范：
* 所有的字母都是小写的，单词之间使用下划线分开
* 只要声明了函数，并且我的主函数够得着其他函数。

```rust
fn main() {
    println!("hello world");
    another_function();
}

fn another_function(){
    println!("another_function");
}
```

## 函数参数

**形参**（parameters）

**实参**（argumens）

在函数的签名里，必须声明每个参数的类型。

```rust
fn main() {
    println!("hello world");
    another_function(5, 6); // argument
}

fn another_function(x: i32, y: i32){
    println!("The value of x is: {}", x); // parameter
    println!("The value of y is: {}", y); // parameter
}
```

## 函数的返回值

在`->`符号后边声明函数返回值的类型，但是不可以为返回值命名

在`Rust`里面，返回值就是函数体最后一个表达式的值。

若想提前返回，需使用`return`关键字，并指定一个值。
* 大多数函数都是默认使用最后一个表达式作为返回值

```rust
fn plus_five(x: i32) -> i32{
    5 + x
    // 5 + x; // 返回空的tuple
}

fn main(){
    let x = plus_five(6);

    println!("The value of x is: {}", x);
}
```

## 注释

```rust
// This is a function

/* sdvwvweve
qvwv
*/
fn plus_five(x: i32) -> i32{
    5 + x
}

fn main(){
    let x = plus_five(6);
    println!("The value of x is: {}", x);
}
```
> 还有一种注释是文档注释