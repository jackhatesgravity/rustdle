extern crate term;

use std::io::{self, Write};
use std::process::Command;
use regex::Regex;
use term::{StderrTerminal, StdoutTerminal};

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

fn print_with_colour_mask(exact_mask: [u8; 5], general_mask: [u8; 5], word: &String) {
    // Create a mutable terminal for manipulating colours.
    let mut terminal = term::stdout().unwrap();
    for n in 0..5 {
        if exact_mask[n] == 1 {
            terminal.fg(term::color::BRIGHT_GREEN).unwrap();
            print!("{}", word.chars().nth(n).unwrap());
            terminal.reset().unwrap();
        } else if general_mask[n] == 1 {
            terminal.fg(term::color::BRIGHT_YELLOW).unwrap();
            print!("{}", word.chars().nth(n).unwrap());
            terminal.reset().unwrap();
        } else {
            print!("{}", word.chars().nth(n).unwrap());
        }
    }
    io::stdout().flush().expect("Failed to flush stdout!");
    println!();
}


fn main() {
    /*** REGEX LAND START ***/

    // Regex migraine time.
    // let re = Regex::new("([flour])").unwrap();

    // // Basic match.
    // let mat = re.find(hay).unwrap();
    // println!("{:?}", mat);

    // // Match all instances.
    // let matches: Vec<_> = re.find_iter(hay).map(|m| m.as_str()).collect();
    // println!("{:?}", matches);

    // // Another alternative.
    // let mat = re.is_match(hay).unwrap();
    // println!("{:?}", mat);

    // I'm overthinking this. I can just compare like for like in a loop for the EXACT match, then use regex for the GENERAL case?

    /*** REGEX LAND END ***/

    // Track the number of rounds.
    let mut turn = 0;
    let mut guesses: [String; 5] = Default::default(); // Initialise with empty Strings

    loop {

        // Initialise and set masks to 0.
        let mut exact_match_mask: [u8; 5] = Default::default(); // To mut or not to mut?
        let mut general_match_mask: [u8; 5] = Default::default(); // Mut. Definitely mut.

        if turn < 5 {
            print!("[Round {}] Enter your guess: ", turn + 1);
            io::stdout().flush().expect("Failed to flush stdout!"); // Flush to make sure we're outputting.

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

            let guess = guess.trim().to_string(); // Trim whitespace and convert to String.

            /*** Here comes the fun bit involving the masks... (Ugly code warning) ***/
            'exact: for n in 0..5 {
                if guess.chars().nth(n).unwrap() == WORD_OF_THE_DAY.chars().nth(n).unwrap() {
                    exact_match_mask[n] = 1;
                }
            }

            'general: for n in 0..5 {
                for m in 0..5 {
                    if guess.chars().nth(n).unwrap() == WORD_OF_THE_DAY.chars().nth(m).unwrap() {
                        general_match_mask[n] = 1;
                    }
                }
            }

            guesses[turn] = guess.clone(); // Store the guess in the array.
            println!("Your guesses: {:?}", guesses);
            print_with_colour_mask(exact_match_mask, general_match_mask, &guess);

            // I can either order it so the exact mask takes priority, or nand the exact against the general to remove double-ups.

            // println!("DEBUG::EXACT_MASK - {:?}", exact_match_mask);
            // println!("DEBUG::GENERAL_MASK - {:?}", general_match_mask);

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
