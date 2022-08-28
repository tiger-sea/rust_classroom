struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rec = Rectangle {
        width: 10,
        height: 5
    };
    // 引数は参照で渡す,所有権を渡したくないから.
    println!("The area of the rectangle is {}", area(&rec));
}

fn area(dementions: &Rectangle) -> u32 {
    dementions.height * dementions.width
}