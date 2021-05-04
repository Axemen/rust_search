use glob::glob;
use std::collections::HashMap;

pub fn find_files(path: String) -> Vec<std::path::PathBuf> {
    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    let mut files_found: u32 = 0;
    println!("Finding Files");
    for path in glob(&path).expect("Failed to read glob pattern") {
        match path {
            Ok(path) => {
                paths.push(path);
                files_found += 1;
                print!("Found {} Files\r", files_found);
            }
            Err(e) => println!("{:?}", e)
        }
    }
    println!("Found {} Files", files_found);
    return paths;
}

pub fn count_tokens(tokens: Vec<String>) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for token in tokens {
        let t = token.to_owned().to_owned();
        if !counts.contains_key(&token) {
            counts.insert(token, 1);
        } else {
            counts[&token] = counts[&token] + 1;
        }
    }
    return counts;
}