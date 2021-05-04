use std::collections::HashMap;
use std::fs;
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

pub trait Searcher {
    fn index_document(self, text: String);
    fn index_file(self, path: String) -> Result<(), io::Error>;
    fn fast_cosine_scores(self, term: String) -> HashMap<i32, f32>;
    // fn lookup(self, terms: Vec<String>) -> Vec<String>;
    // fn remove(self, term: String) -> Option<()>;
}

impl Searcher for Search {
    fn index_document(mut self, text: String) {
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
    }

    fn index_file(self, path: String) -> Result<(), io::Error> {
        let text = fs::read_to_string(path).expect("Failed to read file");
        self.index_document(text);
        return Ok(());
    }

    fn fast_cosine_scores(self, term: String) -> HashMap<i32, f32> {
        let mut scores: HashMap<i32, f32> = HashMap::new(); // document_id -> tfidf_score
        let N = self.index.len() as f32; // Number of terms in the index
        let df = self.index[&term].len() as f32; // Number of documents this term is in

        for doc in &self.index[&term] {
            let tfidf = doc.count as f32 * (N / df).ln() + 1.0;
            *scores.entry(doc.document_id).or_insert(0.0) += tfidf;
        }
        return scores;
    }
}
