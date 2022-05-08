# struct

什么是struct？
> struct，结构体。自定义的数据类型，为相关联的值命名，打包`=>`有意义的组合。

## 定义struct

使用`struct`关键字，并为整个`struct`命名。

在花括号内，为所有**字段（Filed）**定义名称和类型。

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
## 实例化struct

想要使用`struct`，需要创建`struct`的实例：
* 为每个字段指定具体值
* 无需按声明的顺序进行指定

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User{
        email: String::from("1439313331@qq.com"),
        username: String::from("LinJing"),
        active: true,
        sign_in_count: 556,
    };
}
```
取得`struct`里面的某个值：（使用点标记法）
```rust
user1.email = String::from("another@qq.com");
```
> 一旦`struct`的实例是可变的，那么实例中所有的字段都是可变的。

`struct`作为函数的返回值：
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```
字段初始化简写：
```rust
// 当字段名与字段值对应变量名相同时，就可以使用字段初始化简写的方式：
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
`struct`更新的语法：
```rust
// 不采用更新语法
let user2 = User {
    email: String::from("another@qq.com"),
    username: String::from("another"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

// 采用更新语法
let user2 = User {
    email: String::from("another@qq.com"),
    username: String::from("another"),
    ..user1 // 剩下的没有被赋值的实例
};
```
## Tuple struct
可定义类型tuple的struct，叫做tuple struct。
* Tuple struct整体有个名，但里面的元素没有名
* 适用：想给整个tuple起名，并让它不同于其它tuple，且不需要给每个元素起名。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
// black和origin是不同的类型，是不同的tuple struct的实例。
```
Unit-Like Struct：

可以定义没有任何字段的`struct`，叫做`Unit-Like struct`（因为与()单元类型雷系），适用于需要在某个类型上实现某个trait，但是有没有想要存储的数据。

## struct数据的所有权
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
这里的字段使用了`String`而不是`&str`：
* 该实例拥有其所有的数据
* 只要`struct`实例是有效的，那么内部的字段数据也是有效的

`struct`里可以存放引用，但需要使用生命周期
* 生命周期保证只要`struct`实例是有效的，那么里面的引用也是有效的。
* 如果`struct`里面存储引用，而不使用生命周期，就会报错

```rust
struct User {
    username: &str, // missing lifetime specifier
    email: &str, // missing lifetime specifier
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "1439313331@qq.com",
        username: "LinJing",
        active: true,
        sign_in_count: 556,
    }
}
```