use super::*;

use colored::Colorize;

fn included_in_word(word: String, letters: String) -> bool {
    let mut acc: bool = true;

    for letter in letters.chars() {
        if !word.contains(letter) {
            acc = false;
        }
    }

    return acc;
}

fn excluded_in_word(word: String, letters: String) -> bool {
    let mut acc: bool = true;
    for letter in letters.chars() {
        if word.contains(letter) {
            acc = false;
        }
    }
    return acc;
}

fn solve(words: Vec<String>,
             included: String,
             excluded: String) -> Vec<String> {
    let mut possible_words: Vec<String> = Vec::new();

    for word in words.iter() {
        let included_bool = included_in_word(word.to_string(), included.to_string());
        let excluded_bool = excluded_in_word(word.to_string(), excluded.to_string());
        if included_bool && excluded_bool {
            possible_words.push(word.to_string());
        }
    }

    return possible_words;
}

pub fn solve_algorithm(args: parser::Args) {

    let wordlist = readfile::read(args.wordfile);

    if args.args.len() < 2 {
        help::invalid_arguments();
    }

    let words = solve(wordlist, args.args[0].to_string(), args.args[1].to_string());

    for word in words.iter() {
        println!(" > {}", word.green()); 
    }

}
