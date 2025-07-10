// Learn me some good Rust
// Gotta figure out how to get into *hygenic macros*
//
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number:");
    let secret_number = rand::thread_rng().gen_range(1..=50);
    // println!("The secret number is {secret_number}");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("rip");
        println!("you guessed: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }
}
