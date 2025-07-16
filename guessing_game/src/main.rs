
use std::io;
use std::cmp::Ordering; // Uncomment if you want to use ordering for comparisons
use rand::Rng; // Uncomment if you want to use random number generation
use colored::*; // Uncomment if you want to use colored output
fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Please guess a number between 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1 , 101); // Generates a random number between 1 and 100
    // println!("The secret number is: {}", secret_number);

    loop {
        println!(" Enter your guess :");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess.trim());

    match guess.trim().parse::<u32>() {
        Ok(guess_number) => {
            match guess_number.cmp(&secret_number) {
                Ordering::Less => println!("{}","Too small!".red()),
                Ordering::Greater => println!("{}","Too big!".yellow()),
                Ordering::Equal => {
                    println!("{}","You got it!".green());
                    break; // Exit the loop if the guess is correct
                },
            }
        }
        Err(_) => println!("{}", "Please enter a valid number.".blue()),
    }
}
}

