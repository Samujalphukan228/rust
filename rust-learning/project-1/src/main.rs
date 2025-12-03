use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: i32 = match guess
}
