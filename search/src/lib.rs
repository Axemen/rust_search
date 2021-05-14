use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io;

pub mod preprocessing;
pub mod utils;

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub name: String,
    pub length: u32,
}

pub struct SearchResult {
    pub score: f32,
    pub doc_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub index: HashMap<String, HashMap<i32, i32>>,
    pub documents: HashMap<i32, Document>,
}

pub trait Searchable {
    fn new() -> Index;
    fn index_document(&mut self, text: &str, name: &str) -> Option<()>;
    fn index_file(&mut self, path: &str) -> Result<(), io::Error>;
    fn fast_cosine_scores(&self, term: String) -> HashMap<i32, f32>;
    fn rank(&self, terms: Vec<String>) -> Vec<SearchResult>;
    fn remove(&mut self, term: String) -> Result<(), io::Error>;
    fn lookup(&self, terms: Vec<String>);
    fn load(path: &str) -> Result<Index, io::Error>;
    fn save(&self, path: &str) -> Result<(), io::Error>;
    fn bm25_search(&self, query: &str);
}

impl Searchable for Index {
    fn new() -> Index {
        return Index {
            index: HashMap::new(),
            documents: HashMap::new(),
        };
    }

    fn index_document(&mut self, text: &str, name: &str) -> Option<()> {
        let doc_id = self.documents.len() as i32;

        let tokens = preprocessing::tokenize(text);
        let counts = utils::count_tokens(tokens.to_vec());

        // calculate the document length in tokens
        let mut length = 0;
        for (_, v) in &counts {
            length += v;
        }

        self.documents.insert(
            doc_id,
            Document {
                name: name.to_owned(),
                length: length,
            },
        );

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

    fn index_file(&mut self, path: &str) -> Result<(), io::Error> {
        let text = fs::read_to_string(path).expect("Failed to read file");
        self.index_document(&text, path);
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

    fn rank(&self, terms: Vec<String>) -> Vec<SearchResult> {
        let mut all_scores: HashMap<i32, f32> = HashMap::new();

        for term in terms {
            let term_scores = self.fast_cosine_scores(term);
            for (k, v) in term_scores {
                *all_scores.entry(k).or_insert(0.0) += v;
            }
        }

        let mut scores: Vec<SearchResult> = all_scores
            .iter()
            .map(|(k, v)| SearchResult {
                doc_id: *k,
                score: *v / (self.documents[k].length as f32).sqrt(),
            })
            .collect();

        scores.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        scores.reverse();
        return scores;
    }

    fn remove(&mut self, term: String) -> Result<(), io::Error> {
        self.index.remove_entry(&term);
        Ok(())
    }

    fn lookup(&self, terms: Vec<String>) {
        let results = self.rank(terms);
        for result in results {
            println!(
                "{:?}: {:?}",
                self.documents[&result.doc_id].name, result.score
            );
        }
    }

    fn load(path: &str) -> Result<Index, io::Error> {
        let contents: String = fs::read_to_string(path).unwrap();
        return Ok(serde_json::from_str(&contents).unwrap());
    }

    fn save(&self, path: &str) -> Result<(), io::Error> {
        fs::write(path, serde_json::to_string(self).unwrap())?;
        return Ok(());
    }

    fn bm25_search(&self, query: &str) {
        let query_terms = preprocessing::tokenize(query);
        let mut scores: HashMap<i32, f32> = HashMap::new();
        let n = self.documents.len() as f32;
        let k = 1.5;
        let b = 0.75;
        let mut query_idf_sum: f32 = 0.0;

        let avgdl: u32 = self.documents.iter().map(|(_doc_id, doc)| doc.length).sum();

        for term in query_terms {
            let nq = self.index[&term].len() as f32;
            let idf = ((n - nq + 0.5) / (nq + 0.5) + 1.0).ln();

            for (doc_id, tf) in &self.index[&term] {
                let num = (*tf as f32) * (k + 1.0) as f32;
                let den: f32 = (*tf as f32)
                    + k * (1.0 - b + b * (self.documents[&doc_id].length / avgdl) as f32);

                *scores.entry(*doc_id).or_insert(num / den);
            }

            query_idf_sum += idf;
        }

        let mut results: Vec<SearchResult> = scores
            .iter()
            .map(|(k, v)| SearchResult {
                doc_id: *k,
                score: *v * query_idf_sum,
            })
            .collect();

        results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        results.reverse();

        for result in results {
            println!(
                "{:?}: {:?}",
                self.documents[&result.doc_id].name, result.score
            );
        }
    }
}
