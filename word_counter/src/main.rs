use std::{io, collections::HashMap};
#[allow(unused)]
use proconio::input;
fn main() {
    let text = get_string();
    let mut counter_map: HashMap<String, u32> = HashMap::new(); // keyがStringだからword.to_string()しなきゃいけない
    // count part
    for word in text.split_whitespace() {
        let count = counter_map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    
    // debug
    // println!("{:?}", counter_map);
    
    // display result
    for (word, num) in &counter_map {
        println!("{}: {}", word, num);
    }
}   

fn get_string() -> String {
    println!("Input text: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    return s;
}
