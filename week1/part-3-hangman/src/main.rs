// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 10;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0..words.len())].trim())
}

fn chars_to_string(chars: &Vec<char>) -> String {
    chars.into_iter().collect()
}

fn read_char() -> char {
    print!("Please guess a letter: ");
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line."); 
    guess.as_bytes()[0] as char
}

fn verify(secret_word_chars: &Vec<char>, word_show: &Vec<char>, c : char) -> Result<usize, String> {
    for i in 0..secret_word_chars.len() {
        if c == secret_word_chars[i] && word_show[i] == '-' {
            return Ok(i);
        }
    }
    Err(String::from("Sorry, that letter is not in the word"))
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman! Supported by TYZ.");
    println!("--------------------------------------------");
    let mut times : u32 = NUM_INCORRECT_GUESSES;
    let mut left = secret_word_chars.len();
    let mut word_show = vec!['-'; left];
    let mut store = vec![];

    while times > 0 && left > 0 {
        println!("\nThe word so far is {}", chars_to_string(&word_show));
        println!("You have guessed the following letters: {}", chars_to_string(&store));
        println!("You have {} guesses left", times);
        let c = read_char();
        store.push(c);

        match verify(&secret_word_chars, &word_show, c) {
            Ok(i) => {
                word_show[i] = c;
                left = left - 1;
            }
            Err(e) => {
                times = times - 1;
                println!("{}", e);
            }
        }
    }

    if times == 0 {
        println!("\nSorry, you ran out of guesses!");
    }
    if left == 0 {
        println!("\nCongratulations you guessed the secret word: {}!", secret_word);
    }
    println!("--------------------------------------------");
}
