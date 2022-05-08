# 计算长方形面积

## 第一种写法
```rust
fn main() {
    let w = 30;
    let l = 50;

    println!("{}", area(w, l));
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}
```

## 第二种写法

```rust
fn main() {
    let rect = (30, 50);

    println!("{}", area(rect));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
```
## 第三种写法

```rust
#[derive(Debug)] // 派生，为自定义的类型添加很多功能
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", area(&rect)); // 不可变的引用，不会发生所有权的转移

    println!("{:#?}", rect);// std::fmt::display {:?} {:#?}
                            // std::fmt::Debug
                            // #[derive(Debug)]
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
```