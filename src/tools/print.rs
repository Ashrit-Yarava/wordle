use colored::Colorize;

pub fn print_words(words: String, letters: String) {
    for letter in words.chars() {
        if letters.contains(letter) {
            print!("{} ", letter.to_string().red().bold());
        } else {
            print!("{} ", letter.to_string().blue());
        }
    }
    println!();
}
