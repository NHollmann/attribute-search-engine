use tree::HashSetPrefixTree;

use super::SearchIndex;
use crate::{Query, Result, SearchEngineError};
use std::{collections::HashSet, hash::Hash};

mod tree;

pub struct SearchIndexPrefixTree<P> {
    index: HashSetPrefixTree<P>,
}

impl<P: Eq + Hash + Clone> SearchIndexPrefixTree<P> {
    pub fn new() -> Self {
        Self {
            index: HashSetPrefixTree::new(),
        }
    }

    pub fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index.insert(&attribute_value, primary_id);
    }
}

impl<P: Eq + Hash + Clone> SearchIndex<P> for SearchIndexPrefixTree<P> {
    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(_, value) => Ok(self.index.get(value).unwrap_or(HashSet::<P>::new())),
            Query::Prefix(_, _value) => todo!(),
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }
}
