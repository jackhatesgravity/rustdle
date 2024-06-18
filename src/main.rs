extern crate term;

use std::io::{self, Write};
use term::color;

const WORD_OF_THE_DAY: &str = "FLOUR";

fn print_with_colour_mask(exact_mask: &[u8; 5], general_mask: &[u8; 5], word: &str) {
    if let Some(mut terminal) = term::stdout() {
        for (i, c) in word.chars().enumerate() {
            match exact_mask[i] {
                1 => {
                    terminal.fg(color::BRIGHT_GREEN).unwrap();
                    print!("{}", c);
                }
                _ => match general_mask[i] {
                    1 => {
                        terminal.fg(color::BRIGHT_YELLOW).unwrap();
                        print!("{}", c);
                    }
                    _ => print!("{}", c),
                },
            }
            terminal.reset().unwrap();
        }
        io::stdout().flush().expect("Failed to flush stdout!");
        println!();
    }
}


fn main() {
    let mut turn = 0; // Keep track of the number of rounds.
    let mut guesses: [String; 5] = Default::default(); // Initialise with empty Strings.

    while turn < 5 {
        // Initialise and set masks to 0.
        let mut exact_match_mask = [0; 5]; // To mut or not to mut?
        let mut general_match_mask = [0; 5]; // Mut. Definitely mut.

        print!("[Round {}] Enter your guess: ", turn + 1);
        io::stdout().flush().expect("Failed to flush stdout!"); // Flush to make sure we're outputting.

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess = guess.trim().to_string().to_uppercase();

        // Enforce word length.
        if guess.len() != 5 {
            println!("Please enter a five-letter word!");
            continue;
        }

        for (i, (g, w)) in guess.chars().zip(WORD_OF_THE_DAY.chars()).enumerate() {
            if g == w {
                exact_match_mask[i] = 1;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if WORD_OF_THE_DAY.contains(g) && exact_match_mask[i] == 0 {
                general_match_mask[i] = 1;
            }
        }

        guesses[turn] = guess.clone(); // Store the guess in the array.
        // println!("Your guesses: {:?}", guesses);
        print_with_colour_mask(&exact_match_mask, &general_match_mask, &guess);

        if guess == WORD_OF_THE_DAY {
            println!("You guessed correctly!");
            break;
        } else {
            println!("Try again!");
            turn += 1;
        }

        if turn == 5 {
            println!("Game over!");
        }
    }
}
