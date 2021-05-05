use search::utils::find_files;
use search::*;
use std::fs;

fn main() {
    let mut search: Index = Index::new();

    let paths = find_files(r"E:\data\opinions\**\*".to_string());

    for path in &paths[0..10] {

        let text = fs::read_to_string(path).expect("Could not read file");
        let name = path.to_str().unwrap().to_owned();
        search.index_document(text, name);
        let v = &search.documents[&0].name;
        println!("{:?}", v);
        break
    }
}
