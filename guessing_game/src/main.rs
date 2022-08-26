use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    // for cheating lol
    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please Input your guess.");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}