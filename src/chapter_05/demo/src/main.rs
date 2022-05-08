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