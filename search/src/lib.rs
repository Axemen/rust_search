use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

pub mod index;
pub mod preprocessing;
pub mod utils;

struct ScoredDocument {
    id: i32,
    score: i32,
}

pub struct TFDocument {
    document_id: i32,
    count: i32,
}

pub struct Search {
    pub index: HashMap<String, Vec<TFDocument>>,
}

pub trait Searcher {
    fn new() -> Search;
    fn index_document(self, text: String) -> Option<()>;
    fn index_file(self, path: String) -> Result<(), io::Error>;
    fn fast_cosine_scores(&self, term: String) -> HashMap<i32, f32>;
    fn lookup(self, terms: Vec<String>) -> Vec<i32>;
    // fn remove(self, term: String) -> Option<()>;
}

impl Searcher for Search {
    fn new() -> Search {
        return Search {
            index: HashMap::new(),
        };
    }

    fn index_document(mut self, text: String) -> Option<()> {
        let tokens = preprocessing::tokenize(text);
        let counts = utils::count_tokens(tokens.to_vec());
        for token in tokens {
            let doc = TFDocument {
                count: counts[&token] as i32,
                document_id: self.index.len() as i32,
            };
            self.index.entry(token).or_insert(Vec::new()).push(doc);
        }
        return Some(());
    }

    fn index_file(self, path: String) -> Result<(), io::Error> {
        let text = fs::read_to_string(path).expect("Failed to read file");
        self.index_document(text);
        return Ok(());
    }

    fn fast_cosine_scores(&self, term: String) -> HashMap<i32, f32> {
        let mut scores: HashMap<i32, f32> = HashMap::new(); // document_id -> tfidf_score
        let n = self.index.len() as f32; // Number of terms in the index
        let df = self.index[&term].len() as f32; // Number of documents this term is in

        for doc in &self.index[&term] {
            let tfidf = doc.count as f32 * (n / df).ln() + 1.0;
            *scores.entry(doc.document_id).or_insert(0.0) += tfidf;
        }
        return scores;
    }

    fn lookup(self, terms: Vec<String>) -> Vec<i32> {
        let mut all_scores: HashMap<i32, f32> = HashMap::new();
        for term in terms {
            let term_scores = self.fast_cosine_scores(term);
            for (k, v) in term_scores {
                *all_scores.entry(k).or_insert(0.0) += v;
            }
        }

        let mut scores: Vec<ScoredDocument> = all_scores
            .iter()
            .map(|(k, v)| ScoredDocument { id: *k, score: *v as i32 })
            .collect();

        scores.sort_by_key(|a| a.score);
        return scores.iter().map(|x| x.id).collect();
    }
}
