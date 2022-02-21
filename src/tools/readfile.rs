use std::fs;

pub fn read(file: String) -> Vec<String> {
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file.");

    let mut words: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    
    // Remove the last word if it is just a blank line.
    if words[words.len() - 1] == "" {
        words.remove(words.len() - 1);
    }

    return words;

}
