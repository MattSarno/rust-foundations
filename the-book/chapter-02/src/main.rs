use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Welcome to my guessing game!");

    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("\nPlease input a guess between 1 and 100:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let input = guess.trim().to_uppercase();
        if input == "Q" || input == "QUIT" || input == "EXIT" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("\nYou guessed: {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Oops, too small."),
            Ordering::Greater => println!("Nope! Sorry too Big!"),
            Ordering::Equal => {
                println!("Correct, You Win!");
                break;
            }
        }
    }    
}
