const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6; // 不可变的变量不可赋值
    println!("The value of x is {}", x);

    println!("The const value is {}", MAX_POINTS);

    let y = 5; // 声明一个不可变的变量
    // y = y + 1; // 报错，不可变的变量无法赋值
    let y = y + 1; // shadow
    let y = y * 2; // shaow

    println!("The result of y is {}", y);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The length of spaces is {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("The guess is number {}", guess);
    let q = 'z';
    let w: char = 'q';
    let e = '😀';
    println!("{}, {}, {}", q, w, e);
}