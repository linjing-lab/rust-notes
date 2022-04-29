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
        match io::stdin().read_line(&mut guess){
            Ok(string) => string,
            Err(_) => {
                println!("无法读取行");
                break;
            }
        };

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