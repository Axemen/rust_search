use search::utils::find_files;
use search::index::TFDocument;
use std::collections::HashMap;
use search::preprocessing::tokenize;
use std::fs;

fn main() {
    let paths = find_files(r"E:\data\opinions\**\*".to_string());

    let mut index: HashMap<String, TFDocument> = HashMap::new();

    for path in paths {

        let text = fs::read_to_string(path).expect("Could not read file");
        // let mut tokens: Vec<String> = Vec::new();
        let tokens: Vec<String> = tokenize(text);
        

        println!("{}", &tokens[0]);
        break
    }
}
