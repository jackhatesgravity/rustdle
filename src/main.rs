extern crate term;

use std::{fs};
use std::io::{self, Write};
use rand::Rng;
use term::color;

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
                        terminal.fg(color::BRIGHT_MAGENTA).unwrap();
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

fn game_loop(word: &str) {
    let word_of_the_day: &str = word;
    let mut turn = 0; // Keep track of the number of rounds.
    let mut guesses: [String; 5] = Default::default(); // Initialise with empty Strings.

    while turn < 5 {
        // Initialise and set masks to 0.
        let mut exact_match_mask = [0; 5]; // To mut or not to mut?
        let mut general_match_mask = [0; 5]; // Mut. Definitely mut.

        print!("[Round {}] Enter your guess: ", turn + 1);
        io::stdout().flush().expect("Failed to flush stdout!"); // Flush to make sure we're outputting.

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess = guess.trim().to_string().to_uppercase();

        // Enforce word length.
        if guess.len() != 5 {
            println!("Please enter a five-letter word!");
            continue;
        }

        for (i, (g, w)) in guess.chars().zip(word_of_the_day.chars()).enumerate() {
            if g == w {
                exact_match_mask[i] = 1;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if word_of_the_day.contains(g) && exact_match_mask[i] == 0 {
                general_match_mask[i] = 1;
            }
        }

        guesses[turn] = guess.clone(); // Store the guess in the array.
        // println!("Your guesses: {:?}", guesses);
        print_with_colour_mask(&exact_match_mask, &general_match_mask, &guess);

        if guess == word_of_the_day {
            println!("You guessed correctly!");
            break;
        } else {
            println!("Try again!");
            turn += 1;
        }

        if turn == 5 {
            println!("Game over! The correct word was: {word_of_the_day}");
        }
    }
}

fn read_words_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let data = fs::read_to_string(file_path)?;
    Ok(data.lines().map(|line| line.to_string()).collect())
}

fn main() {
    let words = read_words_from_file("src/words.txt").unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    });

    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0..words.len());

    let rand_word = &words[rand_num].to_uppercase();
    game_loop(rand_word);
}
