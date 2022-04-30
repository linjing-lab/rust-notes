# 控制流

## if表达式

`if`表达式允许您根据条件来执行不同的代码分支
* 这个条件必须是bool类型

`if`表达式中，与条件相关联的代码块叫分支（arm）

在后边可以加上一个`else`表达式（可选的）

> ruby, javascript, C++会将非`bool`类型转换为`bool`类型。

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }
}
```

### 使用`else if`处理多重条件

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is dividable by 4");
    }else if number % 3 == 0 {
        println!("number is dividable by 3");
    } else if number % 2 == 0 {
        println!("number is dividable by 2");
    }else {
        println!("number is not dividable by 4, 3, 2");
    }
}
```
如果使用多于一个`else if`，那么最好使用`match`来重构代码。

### 在let语句中使用if

因为`if`是一个表达式，所以可以将它放在`let`语句中等号的右边。

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

## 循环

Rust提供了3种循环：`loop`，`while`，`for`。

### loop循环

loop关键字告诉Rust反复的执行一块代码，直到你喊停

可以在loop循环中使用`break`关键字来告诉程序停止循环

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is : {}", result);
}
```

### while条件循环

一种常见的循环模式是每次执行循环体之前都判断一次条件。

`while`条件循环为这种模式而生。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}
```

### for循环遍历集合

可以使用while或loop来遍历集合，但是易错且低效。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

使用`for`循环更简洁紧凑，它可以针对集合中的每个元素来执行一些代码

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
// a.iter()可以返回一个数组可以遍历的东西
```

由于`for`循环的安全、简洁性，所以它在Rust里用的最多。

## Range

标准库提供，指定一个开始数字和一个结束数字，`range`可以生成它们之间的数字（不含结束）

`rev`可以反转`Range`。

```rust
```