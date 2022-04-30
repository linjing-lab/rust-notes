fn plus_five(x: i32) -> i32{
    5 + x
}

fn main(){
    let x = plus_five(6);

    println!("The value of x is: {}", x);
}