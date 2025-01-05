use super::SearchIndex;
use crate::{Query, QueryValue, Result, SearchEngineError};
use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
};

pub struct SearchIndexRange<P: Eq + Hash + Clone> {
    index: BTreeMap<String, HashSet<P>>,
}

impl<P: Eq + Hash + Clone> SearchIndexRange<P> {
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
        }
    }
}

impl<P: Eq + Hash + Clone> SearchIndex<P> for SearchIndexRange<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(_, QueryValue::Str(value)) => Ok(self
                .index
                .get(value)
                .cloned()
                .unwrap_or(HashSet::<P>::new())),
            Query::Exact(_, _) => Err(SearchEngineError::MismatchedQueryType),
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }
}
