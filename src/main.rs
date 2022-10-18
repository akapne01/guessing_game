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
        let user_guess = get_user_input();

        // Print for debugging reasons. Remove on final version
        println!("You guessed: {user_guess}");

        // Compare the guess with secret number
        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/// Funtion that get's user input from the console and returns it as a string
fn get_user_input() -> u32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Trim is required to strip the new line character \n or in Windows carriage return \r\n, becasue to enter answer, user needs to press enter
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type a number between 1 and 100, inclusive!");
    guess
}

/// Returns a random number in range from 1 to 100, inclusive of both.
fn generate_a_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
