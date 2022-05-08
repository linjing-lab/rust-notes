# struct的方法

方法和函数类似：fn关键字、名称、参数、返回值

方法与函数不同之处：
* 方法是在`struct`（或`enum`、`trait`对象）的上下文中定义
* 第一个参数是`self`，表示方法被调用的`struct`实例

```rust
#[derive(Debug)] // 派生，为自定义的类型添加很多功能
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // 不可变的引用，不会发生所有权的转移
        self.width * self.length
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", rect.area());

    println!("{:#?}", rect);
}
```

## 定义方法

在`impl`（implementation）块里定义方法，方法的第一个参数可以是`&self`，也可以获得其所有权或可变借用。

方法调用的运算符：

C/C++: 
* 调用对象上的方法：object.something()
* 调用指针指向的对象上的方法: object->something()和(*object).something()一样

Rust没有`->`运算符，但是Rust会自动引用或解引用（在调用方法时就会发生这种行为）

在调用方法时，Rust根据情况自动添加`&`、`&mut`或`*`，以便`object`可以匹配方法的签名。
```rust
p1.distance(&p2);
// (&p1).distance(&p2);
```
## 方法参数

```rust
#[derive(Debug)] // 派生，为自定义的类型添加很多功能
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // 不可变的引用，不会发生所有权的转移
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    println!("{}", rect1.area());
    println!("{:#?}", rect1);
}
```

## 关联函数

可以在`impl`块里定义不把`self`作为第一个参数的函数，它们叫关联函数（不是方法）
* `String::from`

```rust
#[derive(Debug)] // 派生，为自定义的类型添加很多功能
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // 不可变的引用，不会发生所有权的转移
        self.width * self.length
    }
    // 能否容纳另外一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let s = Rectangle::square(20);

    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };
    println!("{}", s.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    println!("{}", rect1.area());
    println!("{:#?}", rect1);
}
```