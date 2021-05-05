use std::collections::HashMap;
use std::fs;
use std::io;

pub mod preprocessing;
pub mod utils;

pub struct Document {
    pub name: String,
}

pub struct Index {
    pub index: HashMap<String, HashMap<i32, i32>>,
    pub documents: HashMap<i32, Document>,
}

pub trait Searchable {
    fn new() -> Index;
    fn index_document(&mut self, text: String, name: String) -> Option<()>;
    fn index_file(&mut self, path: String) -> Result<(), io::Error>;
    fn fast_cosine_scores(&self, term: String) -> HashMap<i32, f32>;
    fn rank(&self, terms: Vec<String>) -> Vec<i32>;
    fn remove(&mut self, term: String) -> Result<(), io::Error>;
    fn lookup(&self, terms: Vec<String>);
}

impl Searchable for Index {
    fn new() -> Index {
        return Index {
            index: HashMap::new(),
            documents: HashMap::new(),
        };
    }

    fn index_document(&mut self, text: String, name: String) -> Option<()> {
        let doc_id = self.documents.len() as i32;
        self.documents.insert(doc_id, Document { name: name });

        let tokens = preprocessing::tokenize(text);
        let counts = utils::count_tokens(tokens.to_vec());

        for token in tokens {
            *self
                .index
                .entry(token)
                .or_insert(HashMap::new())
                .entry(doc_id)
                .or_insert(0) += counts[&token] as i32;
        }
        return Some(());
    }

    fn index_file(&mut self, path: String) -> Result<(), io::Error> {
        let text = fs::read_to_string(&path).expect("Failed to read file");
        self.index_document(text, path);
        return Ok(());
    }

    fn fast_cosine_scores(&self, term: String) -> HashMap<i32, f32> {
        let mut scores: HashMap<i32, f32> = HashMap::new(); // document_id -> tfidf_score
        let n = self.index.len() as f32; // Number of terms in the index
        let df = self.index[&term].len() as f32; // Number of documents this term is in

        for (id, count) in &self.index[&term] {
            let tfidf = *count as f32 * (n / df).ln() + 1.0;
            *scores.entry(*id).or_insert(0.0) += tfidf;
        }
        return scores;
    }

    fn rank(&self, terms: Vec<String>) -> Vec<i32> {

        let mut all_scores: HashMap<i32, f32> = HashMap::new();

        for term in terms {
            let term_scores = self.fast_cosine_scores(term);
            for (k, v) in term_scores {
                *all_scores.entry(k).or_insert(0.0) += v;
            }
        }

        let mut scores: Vec<Vec<i32>> = all_scores
            .iter()
            .map(|(k, v)| vec![*k, *v as i32])
            .collect();
        scores.sort_by_key(|x| x[1]);
        return scores.iter().map(|x| x[0]).collect();
    }

    fn remove(&mut self, term: String) -> Result<(), io::Error> {
        self.index.remove_entry(&term);
        Ok(())
    }

    fn lookup(&self, terms: Vec<String>) {
        let ranks = self.rank(terms);
        for id in ranks {
            println!("{:?}", self.documents[&id].name);
        }
    }
}
