// Prints the help method.

use std::process;
use colored::Colorize;

pub fn help() {
    let help_message = r#"

    Usage: wordle [option] [wordlife] [arguments]

    Options:
        * solve [included-letters] [excluded-letter]
        
        Find words with the designated letters included and excluded from the word.

        * help

        Print this help message.

"#;
    println!("{}", help_message);
    process::exit(0);
}

// Prints Invalid argument list.
pub fn invalid_arguments() {
    println!("{}", "Invalid Number of Arguments".red());
    process::exit(0);
}
