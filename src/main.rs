// This is a guessing game that expects user to enter the guess
// in the console.

use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Guess the number game!");
    println!("Please eneter a random number from 1 to 100 inclusive");

    let secret_number = generate_a_random_number();
    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

/// Returns a random number in range from 1 to 100, inclusive of both.
fn generate_a_random_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}
