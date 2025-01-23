use super::{query_string_to_type, SearchIndex};
use crate::{Query, Result, SearchEngineError};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

/// SearchIndexHashMap is a index baked by a HashMap that can only match exact attribute values.
///
/// # Example
/// ```
/// use attribute_search_engine::{SearchIndex, SearchIndexHashMap};
/// use std::collections::HashSet;
/// use attribute_search_engine::Query;
///
/// let mut index_city = SearchIndexHashMap::<usize, String>::new();
/// index_city.insert(0, "Berlin".into());
/// index_city.insert(1, "New York".into());
/// index_city.insert(2, "Madrid".into());
///
/// let result = index_city.search(&Query::Exact("<not used>".into(), "New York".into())).unwrap();
/// assert_eq!(result, HashSet::from_iter(vec![1]));
/// ```
pub struct SearchIndexHashMap<P, V> {
    index: HashMap<V, HashSet<P>>,
}

impl<P, V> SearchIndexHashMap<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Eq + Hash + FromStr + 'static,
{
    /// Create a new SearchIndexHashMap.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, primary_id: P, attribute_value: V) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }
}

impl<P: Clone, V: Eq + Hash + FromStr> SearchIndex<P> for SearchIndexHashMap<P, V> {
    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        let value_str = match query {
            Query::Exact(_, value) => Ok(value),
            _ => Err(SearchEngineError::UnsupportedQuery),
        }?;
        let value: V = query_string_to_type(value_str)?;
        Ok(self
            .index
            .get(&value)
            .cloned()
            .unwrap_or(HashSet::<P>::new()))
    }
}
