use super::{SearchIndex, SearchIndexBuilder};
use crate::{Query, Result, SearchEngineError};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct SearchIndexExact<P, V> {
    index: HashMap<V, HashSet<P>>,
}

impl<P, V> SearchIndexExact<P, V> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
}

impl<P, V> SearchIndexBuilder<P, V> for SearchIndexExact<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Eq + Hash + Clone + From<String> + 'static,
{
    fn insert(&mut self, primary_id: P, attribute_value: V) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    fn build(self) -> Box<dyn SearchIndex<P>> {
        Box::new(self)
    }
}

impl<P: Clone, V: Eq + Hash + From<String>> SearchIndex<P> for SearchIndexExact<P, V> {
    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        let attribute_value_str = match query {
            Query::Exact(_, value) => Ok(value),
            // Query::Exact(_, _) => Err(SearchEngineError::MismatchedQueryType), // TODO
            _ => Err(SearchEngineError::UnsupportedQuery),
        }?;
        let attribute_value: V = attribute_value_str.to_owned().into();
        Ok(self
            .index
            .get(&attribute_value)
            .cloned()
            .unwrap_or(HashSet::<P>::new()))
    }
}
