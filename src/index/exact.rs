use super::{query_string_to_type, SearchIndex, SearchIndexBuilder};
use crate::{Query, Result, SearchEngineError};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

/// SearchIndexExact is a index that can only match exact attribute values.
///
/// # Example
/// ```
/// use attribute_search_engine::{SearchIndex, SearchIndexBuilder, SearchIndexExact};
/// use std::collections::HashSet;
/// use attribute_search_engine::Query;
///
/// let mut index_city = SearchIndexExact::<usize, String>::new();
/// index_city.insert(0, "Berlin".into());
/// index_city.insert(1, "New York".into());
/// index_city.insert(2, "Madrid".into());
///
/// let index_city = index_city.build();
/// let result = index_city.search(&Query::Exact("<not used>".into(), "New York".into())).unwrap();
/// assert_eq!(result, HashSet::from_iter(vec![1]));
/// ```
pub struct SearchIndexExact<P, V> {
    index: HashMap<V, HashSet<P>>,
}

impl<P, V> SearchIndexExact<P, V> {
    /// Create a new SearchIndexExact.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
}

impl<P, V> SearchIndexBuilder<P, V> for SearchIndexExact<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Eq + Hash + FromStr + 'static,
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

impl<P: Clone, V: Eq + Hash + FromStr> SearchIndex<P> for SearchIndexExact<P, V> {
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
