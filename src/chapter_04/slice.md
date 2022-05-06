# 切片

Rust的另外一种不持有所有权的数据类型：切片（slice）

编写一个函数：
* 它接收字符串作为参数
* 返回它在这个字符串里找到的第一个单词
* 如果函数没找到任何空格，那么整个字符串就被返回

```rust
fn main() {
    let s = String::from("Hello world");
    let word_index = first_world(&s);
    println!("{}", word_index);
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```
## 字符串切片

字符串切片是指向字符串种一部分内容的引用。

形式：[开始索引..结束索引]
* 开始索引是切片起始位置的索引值
* 结束索引是切片终止位置的下一个索引值

```rust
fn main() {
    let s = String::from("Hello world");
    let hello = &s[0..5]; // hello = &s[..5];
    let world = &s[6..11]; // world = &s[6..]

    let whole = &s[..];
    println!("{}, {}", hello, world);
}
```

字符串切片的范围索引必须发生在有效的UTF-8字符边界内。

如果尝试从一个多字节的字符中创建字符串切片，程序会报错并退出。

## 字符串字面值是切片

字符串字面值被直接存储在二进制程序中。
```rust
let s = "hello, world"; // &str
```
变量s的类型是&str，它是一个指向二进制程序特定位置的切片
* &str是不可变引用，所以字符串字面值也是不可变的

## 将字符串切片作为参数传递

有经验的Rust开发者会采用&str作为参数类型，因为这样可以同时接收String和&str类型的参数了

```rust
fn first_world(s: &String) -> &str {

}

fn first_world(s: &str) -> &str {

}
```
* 使用字符串切片，直接调用该函数
* 使用String，可以创建一个完整的String切片来调用该函数

定义函数时使用字符串切片来代替字符串引用会使我们的API更加通用，且不会损失任何功能。

```rust
fn main() {
    // let my_string = String::from("Hello world");
    // let word_index = first_world(&my_string[..]);

    let my_string_literal = "hello world";
    let word_index = first_world(my_string_literal);
    println!("{}", word_index);
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```

## 其它类型的切片

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}", slice[0]);
}
```