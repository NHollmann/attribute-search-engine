use super::SearchIndex;
use crate::{Query, Result, SearchEngineError};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct SearchIndexExact<P: Eq + Hash + Clone> {
    index: HashMap<String, HashSet<P>>,
}

impl<P: Eq + Hash + Clone> SearchIndexExact<P> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
}

impl<P: Eq + Hash + Clone> SearchIndex<P> for SearchIndexExact<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        let attribute_value = match query {
            Query::Exact(_, value) => Ok(value),
            // Query::Exact(_, _) => Err(SearchEngineError::MismatchedQueryType), // TODO
            _ => Err(SearchEngineError::UnsupportedQuery),
        }?;
        Ok(self
            .index
            .get(attribute_value)
            .cloned()
            .unwrap_or(HashSet::<P>::new()))
    }
}
