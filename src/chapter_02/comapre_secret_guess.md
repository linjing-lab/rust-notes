# 猜数游戏: 比较猜测数字与神秘数字

## 类型转换

* shadow机制， trim()去掉前后空格（包括回车），parse()将字符串解析成数字类型u32（显式声明）

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
// io::Result Ok, Err，如果Err，会出现expect中的信息
```

## std::cmp::Ordering

&secert_number: 接受一个整数类型的引用

```rust
match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

match表达式一般有多个arm来组成，match后面值与某一个值匹配上，则会打印下一个arm的值。

## 多次猜测

```rust
use std::io; // prelude
use rand::Rng; // trait or interface
use std::cmp::Ordering;

fn main() {
    println!("猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("这个神秘数字是: {}", secret_number)
    loop {
        println!("猜测一个数");
        let mut guess = String::new(); // i32
        io::stdin().read_line(&mut guess).expect("无法读取行");

        /*
        match io::stdin().read_line(&mut guess){
            Ok(string) => string,
            Err(_) => {
                println!("无法读取行");
                break;
            }
        };
        */

        // shadow
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"), //arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
```