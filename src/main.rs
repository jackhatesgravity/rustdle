extern crate term;

use rand::Rng;
use rustdle::{game_loop, read_words_from_file};

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
