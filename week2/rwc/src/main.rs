use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = File::open(filename).expect("Fauilre: open file");
    let mut lines = vec![];
    let n_lines : usize; 
    let mut n_words : usize = 0; 
    let n_chars : usize; 
    for line in io::BufReader::new(file).lines() {
        let line_str = line.unwrap();
        lines.push(line_str);
    }
    n_lines = lines.len();
    for line in &lines {
        for word in line.split_whitespace() {
            n_words += 1;
        }
    }
    n_chars = lines.into_iter().map(|x| x.len()).sum();

    println!("{} {} {} {}", n_lines, n_words, n_chars, filename);
}
