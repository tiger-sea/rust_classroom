fn main() {
    let rec = (10, 5);
    println!("The area of the rectangle is {}", area(rec));
}

fn area(dementions: (u32, u32)) -> u32 {
    dementions.0 * dementions.1
}