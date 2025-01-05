use super::SearchIndex;
use crate::{Query, QueryValue, Result, SearchEngineError};
use std::{collections::HashSet, hash::Hash};
use trie_rs::map::{Trie, TrieBuilder};

pub struct SearchIndexPrefix<P: Eq + Hash + Ord> {
    // TODO we need to build the index. Maybe add a build() function to the trait?
    builder: TrieBuilder<u8, HashSet<P>>,
    index: Option<Trie<u8, HashSet<P>>>,
}

impl<P: Eq + Hash + Ord + Clone> SearchIndexPrefix<P> {
    pub fn new() -> Self {
        Self {
            builder: TrieBuilder::new(),
            index: None,
        }
    }
}

impl<P: Eq + Hash + Ord + Clone> SearchIndex<P> for SearchIndexPrefix<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        let mut hs = HashSet::new();
        hs.insert(primary_id);
        self.builder.push(attribute_value, hs);
    }

    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(_, QueryValue::Str(value)) => {
                Ok(self
                    .index
                    .as_ref()
                    .unwrap() // TODO Do not simply unwrap
                    .exact_match(value)
                    .cloned()
                    .unwrap_or(HashSet::<P>::new()))
            }
            Query::Exact(_, _) => Err(SearchEngineError::MismatchedQueryType),
            Query::Prefix(_, QueryValue::Str(_prefix)) => todo!(),
            Query::Prefix(_, _) => Err(SearchEngineError::MismatchedQueryType),
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }
}
