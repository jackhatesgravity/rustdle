extern crate term;

use std::io::{self, Write};
use regex::Regex;

const WORD_OF_THE_DAY: &str = "flour";

/*
 Brainstorming time.
 I think the way to handle the terminal output for showing the correct letters and correct places is the term crate, for sure.
 In order to do that properly, I think I need a function that can mask certain letters of the output, if that makes sense?
 The comparison happens outside the function, I think I just need to feed the guess, a mask, and a colour.
 The mask is probably an array of 0's and 1's (maybe even just a byte/word) that tells us whether we colour that character or not.

 Initial thought was to call the same function twice, but that means printing the guess to the terminal twice in different colours.
 Maybe instead of a colour, we feed it two masks? That way we can output a single response with the correct feedback.
 While not letting perfect be the enemy of the good, I want to try and avoid simply writing if/else catches for each letter.
 If that's what I have to do to start off, however, then that's what I have to do.
 */

fn main() {

    // Create a mutable terminal for manipulating colours.
    let mut terminal = term::stdout().unwrap();

    // Regex migraine time.
    let re = Regex::new("([flour])").unwrap();

    // Track the number of rounds.
    let mut turn = 0;
    let mut guesses: [String; 5] = Default::default(); // Initialise with empty Strings

    loop {
        println!(); // Blank line at the start.
        if turn < 5 {
            terminal.fg(term::color::BRIGHT_GREEN).unwrap(); // I feel like this is a good target for matching.
            print!("[Round {}] Enter your guess: ", turn + 1);
            terminal.reset().unwrap();
            io::stdout().flush().expect("Failed to flush stdout!"); // Flush to make sure we're outputting.

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

            let guess = guess.trim().to_string(); // Trim whitespace and convert to String.

            guesses[turn] = guess.clone(); // Store the guess in the array.
            println!("Your guesses: {:?}", guesses);

            if guess == WORD_OF_THE_DAY {
                println!("You guessed correctly!");
                break;
            } else {
                println!("Try again!");
                turn += 1;
            }
        } else {
            println!("Game over!");
            break;
        }
    }
}
