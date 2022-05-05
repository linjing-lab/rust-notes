# 所有权与函数

在语义上，将值传递给函数和把值赋给变量是类似的：
* 将值传递给函数将发生移动或复制

```rust
fn main() {
    let s = String::from("Hello World");

    take_ownership(s); // 产生移动，s不再有效

    let x = 5;

    makes_copy(x); // 产生复制，x仍然有效

    println!("x: {}", x);
}

fn take_ownership(some_string: String){
    println!("{}", some_string);
} // some_string被释放

fn makes_copy(some_number: i32){
    println!("{}", some_number);
}
```

## 返回值与作用域

函数在返回值的过程中同样也会发生所有权的转移。

一个变量的所有权总是遵循同样的模式：
* 把一个值给其它变量时就会发生移动
* 当一个包含heap数据的变量离开作用域时，它的值会被drop函数清理，除非数据的所有权移动到另一个变量上了。

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```