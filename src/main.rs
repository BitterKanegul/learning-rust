// Learn me some good Rust
// Gotta figure out how to get into *hygenic macros*
//

use std::io;

fn main() {
    println!("Guess the number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("rip");
    println!("you guessed: {guess}")
}
