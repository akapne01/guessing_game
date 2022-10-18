// This is a guessing game that expects user to enter the guess
// in the console.

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guess the number game!");
    println!("Please eneter a random number from 1 to 100 inclusive");

    let secret_number: u32 = generate_a_random_number();
    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Handle invalid input by only accepting num as valid input and skipping all other types
        // It ignores all errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print for debugging reasons. Remove on final version
        println!("You guessed: {guess}");

        // Compare the guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/// Returns a random number in range from 1 to 100, inclusive of both.
fn generate_a_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
