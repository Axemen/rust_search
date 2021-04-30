use regex::Regex;
use std::fs;

trait Tokenize {
    fn tokenize(self) -> Vec<String>;
}

impl Tokenize for String {
    fn tokenize(self) -> Vec<String> {
        let word_regex = Regex::new(r"\b\w*\b").unwrap();
        let tokens = word_regex.captures_iter(&self).map(|x| x[0].to_string()).collect();
        return tokens;
    }
}

fn main() {
    let contents = read_file("test.txt".to_string());
    let tokens: Vec<String> = contents.tokenize();

    for token in tokens {
        println!("{}", token);
    }
}

fn read_file(file_path: String) -> String {
    let f = fs::read_to_string(file_path).expect("Could not read the file");
    return f.trim().to_string();
}

