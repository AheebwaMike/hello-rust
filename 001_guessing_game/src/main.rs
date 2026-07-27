// my second rust program: Number Guess Game

use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn print_banner() {
    println!("\n╔══════════════════════════════╗");
    println!("║   🎯 Number Guessing Game     ║");
    println!("╚══════════════════════════════╝");
}

fn print_instructions() {
    println!("\nInstructions");
    println!("─────────────");
    println!("• I’m thinking of a number between 1 and 100.");
    println!("• You have 10 chances to guess it.");
    println!("• I’ll tell you if your guess is too high or too low.\n");
}

fn main() {
    print_banner();
    print_instructions();

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut n_guesses: u32 = 10;
    let mut attempt = 1;

    loop {
        println!("┌─ Attempt {attempt} of 10 ─┐");
        println!("│ Chances left: {n_guesses} │");
        println!("└────────────────────────┘");
        print!("Enter your guess: ");
        stdout().flush().expect("Failed to flush output");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  That is not a valid number. Please enter a number between 1 and 100.\n");
                continue;
            }
        };

        match secret_number.cmp(&guess) {
            Ordering::Less => println!("🔼 {guess} is too high. Try again.\n"),
            Ordering::Greater => println!("🔽 {guess} is too low. Try again.\n"),
            Ordering::Equal => {
                println!("✅ Correct! The number was {secret_number}.\n");
                break;
            }
        }

        n_guesses -= 1;
        attempt += 1;

        if n_guesses == 0 {
            println!("💀 You ran out of chances. The number was {secret_number}.\n");
            break;
        }
    }
}
