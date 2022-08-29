#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    
    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}

// これは関連関数(引数に&selfを取らない関数)でメソッドではない
#[allow(unused)]
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { height: size, width: size }
    }
}

fn main() {
    let rec1 = Rectangle { width: 10, height: 5 };
    let rec2 = Rectangle { width: 2, height: 2 };
    let rec3 = Rectangle { width: 100, height: 100 };
    // 引数は参照で渡す,所有権を渡したくないから.
    println!("The area of the rectangle is {}", rec1.area());
    // println!("Rec1 is {:#?}", rec1); // for debug
    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));
}