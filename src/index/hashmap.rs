use super::{string_to_payload_type, SearchIndex};
use crate::{Query, Result, SearchEngineError};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

/// SearchIndexHashMap is a index backed by a HashMap that can match
/// Exact queries.
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
/// let result = index_city.search(&Query::Exact("<unused>".into(), "New York".into()));
/// assert_eq!(result, Ok(HashSet::from_iter(vec![1])));
/// ```
pub struct SearchIndexHashMap<P, V> {
    index: HashMap<V, HashSet<P>>,
}

impl<P, V> Default for SearchIndexHashMap<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Eq + Hash + FromStr + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P, V> SearchIndexHashMap<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Eq + Hash + FromStr + 'static,
{
    /// Creates a new `SearchIndexHashMap`.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::SearchIndexHashMap;
    ///
    /// let index = SearchIndexHashMap::<usize, String>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// Insert a new entry in the index.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::SearchIndexHashMap;
    ///
    /// let mut index = SearchIndexHashMap::<usize, String>::new();
    ///
    /// // You insert an entry by giving a row / primary id and an attribute value:
    /// index.insert(123, "A".into());
    /// // The same row / primary id can have multiple attributes assigned:
    /// index.insert(123, "B".into());
    /// // Add as much entries as you want for as many rows you want:
    /// index.insert(124, "C".into());
    /// ```
    pub fn insert(&mut self, primary_id: P, attribute_value: V) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }
}

impl<P: Clone, V: Eq + Hash + FromStr> SearchIndex<P> for SearchIndexHashMap<P, V> {
    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(_, value_str) => {
                let value: V = string_to_payload_type(value_str)?;
                Ok(self
                    .index
                    .get(&value)
                    .cloned()
                    .unwrap_or(HashSet::<P>::new()))
            }
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_index_exact_string() {
        let mut index = SearchIndexHashMap::<usize, String>::new();
        index.insert(0, "A".into());
        index.insert(0, "B".into());
        index.insert(0, "C".into());
        index.insert(1, "A".into());
        index.insert(1, "B".into());
        index.insert(2, "A".into());

        let result = index.search(&Query::Exact("<not used>".into(), "A".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2])));

        let result = index.search(&Query::Exact("<not used>".into(), "B".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1])));

        let result = index.search(&Query::Exact("<not used>".into(), "C".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0])));

        let result = index.search(&Query::Exact("<not used>".into(), "D".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));
    }

    #[test]
    fn search_index_exact_number() {
        let mut index = SearchIndexHashMap::<usize, i32>::new();
        index.insert(0, 0);
        index.insert(0, 1);
        index.insert(0, 2);
        index.insert(1, 0);
        index.insert(1, 1);
        index.insert(2, 0);

        let result = index.search(&Query::Exact("<not used>".into(), "0".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2])));

        let result = index.search(&Query::Exact("<not used>".into(), "1".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1])));

        let result = index.search(&Query::Exact("<not used>".into(), "2".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0])));

        let result = index.search(&Query::Exact("<not used>".into(), "4".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));
    }

    #[test]
    fn search_index_unsupported_queries() {
        let mut index = SearchIndexHashMap::<usize, i32>::new();
        index.insert(0, 0);

        assert_eq!(
            index.search(&Query::Prefix("<not used>".into(), "0".into())),
            Err(SearchEngineError::UnsupportedQuery)
        );
        assert_eq!(
            index.search(&Query::InRange("<not used>".into(), "0".into(), "1".into())),
            Err(SearchEngineError::UnsupportedQuery)
        );
        assert_eq!(
            index.search(&Query::OutRange(
                "<not used>".into(),
                "0".into(),
                "1".into()
            )),
            Err(SearchEngineError::UnsupportedQuery)
        );
        assert_eq!(
            index.search(&Query::Minimum("<not used>".into(), "0".into())),
            Err(SearchEngineError::UnsupportedQuery)
        );
        assert_eq!(
            index.search(&Query::Maximum("<not used>".into(), "0".into())),
            Err(SearchEngineError::UnsupportedQuery)
        );
        assert_eq!(
            index.search(&Query::Or(vec![])),
            Err(SearchEngineError::UnsupportedQuery)
        );
        assert_eq!(
            index.search(&Query::And(vec![])),
            Err(SearchEngineError::UnsupportedQuery)
        );
        assert_eq!(
            index.search(&Query::Exclude(
                Box::new(Query::Exact("<not used>".into(), "0".into())),
                vec![]
            )),
            Err(SearchEngineError::UnsupportedQuery)
        );
    }
}
