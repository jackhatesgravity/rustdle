use std::io::{self, Write};

const EMPTY_STRING: String = String::new();

fn main() {

    // Track the number of rounds.
    let mut turn = 0;
    let mut guesses: [String; 5] = [EMPTY_STRING; 5];

    loop {
        if turn < 5 {
            print!("[Round {}] Enter your guess: ", turn + 1);
            io::stdout().flush().expect("Failed to flush stdout!");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

            let guess: String = match guess.trim().parse() {
                Ok(str) => str,
                Err(_) => continue,
            };
            guesses[turn] = guess;
            println!("Your guesses: {:?}", guesses);
            turn += 1;
        } else {
            println!("Game over!");
            break;
        }
    }
}
