use super::SearchIndex;
use crate::{Query, Result, SearchEngineError};
use std::{collections::HashSet, hash::Hash};
use trie_rs::map::{Trie, TrieBuilder};

pub struct SearchIndexPrefix<P: Eq + Hash + Clone> {
    // TODO we need to build the index. Maybe add a build() function to the trait?
    builder: TrieBuilder<u8, HashSet<P>>,
    index: Option<Trie<u8, HashSet<P>>>,
}

impl<P: Eq + Hash + Clone> SearchIndexPrefix<P> {
    pub fn new() -> Self {
        Self {
            builder: TrieBuilder::new(),
            index: None,
        }
    }

    pub fn insert(&mut self, primary_id: P, attribute_value: String) {
        let mut hs = HashSet::new();
        hs.insert(primary_id);
        self.builder.push(attribute_value, hs);
    }

}

impl<P: Eq + Hash + Clone> SearchIndex<P> for SearchIndexPrefix<P> {
    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(_, value) => {
                Ok(self
                    .index
                    .as_ref()
                    .unwrap() // TODO Do not simply unwrap
                    .exact_match(value)
                    .cloned()
                    .unwrap_or(HashSet::<P>::new()))
            }
            // Query::Exact(_, _) => Err(SearchEngineError::MismatchedQueryType), TODO
            Query::Prefix(_, _prefix) => todo!(),
            // Query::Prefix(_, _) => Err(SearchEngineError::MismatchedQueryType), TODO
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }
}
