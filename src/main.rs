// Bring IO lib into scope to obtain user input and print the result
use std::io;

// Program entry point
fn main() {
    // Macros that print to the screen 
    println!("Guess the number!");
    println!("Please input your guess.");

    /* Create a variable named 'guess' to store the user input in.
    'mut' allows the variable to be mutable, as they are not by default*/
    let mut guess = String::new(); // String::new(); returns a new instance of a String.

    // stdin function from the 'io' module
    io::stdin()
        // Standard input handle for user input
        .read_line(&mut guess) // takes user input and puts it in a string
        .expect("Failed to read line");

    println!("You guessed: {}", guess);


}
