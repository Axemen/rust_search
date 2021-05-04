use std::collections::HashMap;
use std::io;

pub mod index;
pub mod preprocessing;
pub mod utils;

struct TFDocument {
    document_id: i32,
    count: i32,
}

pub struct Search {
    index: HashMap<String, Vec<TFDocument>>,
}

pub trait search {
    fn index_document(self, text: String) -> Result<(), io::Error>;
    // fn index_file(self, path: String) -> Option<()>;
    // fn fast_cosine_score(self, term: String) -> u32;
    // fn lookup(self, terms: Vec<String>) -> Vec<String>;
    // fn remove(self, term: String) -> Option<()>;
}

impl search for Search {
    fn index_document(mut self, text: String) -> Result<(), io::Error> {
        let tokens = preprocessing::tokenize(text);
        let counts = utils::count_tokens(tokens.to_vec());
        for token in tokens {
            let doc = TFDocument {
                count: counts[&token] as i32,
                document_id: self.index.len() as i32,
            };
            if !self.index.contains_key(&token) {
                self.index.insert(token.to_owned().to_owned(), vec![doc]);
            } else {
                self.index.get_mut(&token).unwrap().push(doc);
            }
        }
        return Ok(());
    }
}
