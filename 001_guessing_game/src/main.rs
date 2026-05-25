// my second rust program: Number Guess Game

use std::io::stdin; // import standard input function from the io crate in the standard library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\n\t\t----------------------------");
    println!("\t\tWelcome to guessing game!");
    println!("\t\t----------------------------");

    println!("INSTRUCTIONS:");
    println!(
        ">> I'm thinking of a number between 1-100.\n>> Let's see how many turns you can taking before guessing my number.\n\n"
    );

    // Important: You won’t just know which traits to use and which methods and 
    // functions to call from a crate (library). Always look up the
    // documentation provided by the crate's documentatation.
    // That said, this line uses the thread_rng function to create a random
    // number generator, which has a method gen_range that takes a range
    // expression.
    // Note: running cargo doc --open will locally build and open the 
    // documentation of all your project dependencies in the browser!
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut n_guesses: u32 = 10;

    // println!("The secret number is {secret_number}");

    loop{
        println!("\nPlease enter your guess: (You have {n_guesses} chances left.)");

        let mut guess = String::new(); // create a mutable variable to store the user's guess

        // lines 20-32 could be compressed to: stdin().read_line(&mut guess).expect("Failed to read line")
        stdin()
            // read_line is a method of the Stdin type, which appends a terminal input to whatever
            // string is passed into it as an argument. This is why we needed to make our guess
            // variable a mutable (cahngeable) string, so that this could be possible.
            // We use &mut guess to reference the variable `guess` without having to copy it in memory.
            // Note. read_line returns a *Result type*.
            .read_line(&mut guess)
            // The Result is an Enum type, which is a type that can be in one of multiple possible states.
            // It's purpose (Result) is to encode error-handling information. expect() is a method of the
            // Result instances, and should an instance have a value of Err, expect will cause the program
            // to crash and return the message that was passed in. If the value is Ok, then expect takes 
            // the return value that ok is holding and returns it for use (the number of bytes in the input).
            // Without using expect(), the code still compiles, but with a warning.
            // Note: The right way to supress a warning is writing error-handling code for it's case.
            // For this simple case, we just need to crash the system if an error occurs, and print a
            // message!
            .expect("Failed to read line");


            // To compare guess and secret number, we need to ensure that both are of the same type,
            // integers in this case (of type u32, for example). To get the integer version of guess, we
            // need to create a new variable, and perform the type conversion on our `guess` string using
            // the parse() method. But first, we trim all white spaces and new lines using trim().
            // Note that here, instead of creating a new variable, we just reasign / shadow the guess variable!
            // Also not that parse returns a Result type.
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That is not a number. Please enter a number.");
                    continue;
                }
            };


            // match has comparison arms, and it looks at the value of the comparison it's given and it's
            // arms and decides which arm matches with this value. The code in that arm runs. In fact, if
            // a matching arm is reached, match stops looking at other arms and immediately runs the code
            // in that arm.
            match secret_number.cmp(&guess) {
                Ordering::Less => println!("{guess} is bigger than my number! Try again"),
                Ordering::Greater => println!("{guess} is smaller than my number! Try again"),
                Ordering::Equal => {
                    println!("You win! My number is indeed {secret_number}");
                    break;
                }
            };

            n_guesses -= 1;

            match n_guesses.cmp(&0) {
                Ordering::Equal => {
                    println!("\n\t\t--------------------------------");
                    println!("\t\tYour chances are over. You lose!");
                    println!("\t\t--------------------------------\n");
                    break
                },
                Ordering::Less => continue,
                Ordering::Greater => continue
            };
    }   
}
