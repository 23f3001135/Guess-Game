// Description: Project 1 - A simple number guessing game in Rust (Learning Rust!)
use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    
    let secret_number: u32 = rand::rng().random_range(1..=100);
    // println!("The secret number is: {}", secret_number);
    let mut number_of_guesses: u32 = 0;
    loop {

        println!("Input your guess: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");
        println!("You guessed: {}", guess);
        number_of_guesses += 1;
        
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Perfect! You guessed the number!");
                println!("You took {} guesses.", number_of_guesses);
                if number_of_guesses <= 5 {
                    println!("You are a pro!");
                } else if number_of_guesses <= 10 {
                    println!("You did well!");
                } else {
                    println!("You can do better!");
                }
                println!("Challenge, do it in less than 5 guesses next time! Wanna try? (y/n)");
                let mut answer:String = String::new();
                io::stdin().read_line(&mut answer).expect("Sorry, Pls enter y for yes or n for no");
                if answer.trim().to_lowercase() == "y" {
                    main(); // Restart the game
                } else {
                    println!("Thanks for playing!");
                    std::process::exit(0); // Exit the loop and end the game
                }
            }
        }
    }
}