// Description: Project 1 - A simple number guessing game in Rust (Learning Rust!)
use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Input your guess: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");
        println!("You guessed: {}", guess);
        
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Perfect! You guessed the number!");
                break;
            }
            _ => println!("Something went wrong!"),
        }
    }
}