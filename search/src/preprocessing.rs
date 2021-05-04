use regex::{Regex};

pub fn tokenize(text: String) -> Vec<String> {
    let word_regex = Regex::new(r"\b\w*\b").unwrap();
    let captures = word_regex.captures_iter(&text.to_lowercase());
    let mut tokens: Vec<String> = Vec::new();
    for cap in captures {
        tokens.push(cap[0].to_string());
    }
    return tokens;
}
