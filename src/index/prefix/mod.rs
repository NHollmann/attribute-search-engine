mod tree;

use super::SearchIndex;
use crate::{Query, Result, SearchEngineError};
use std::{collections::HashSet, hash::Hash};
use tree::HashSetPrefixTree;

/// SearchIndexPrefixTree is a index backed by a prefix tree that can match
/// Exact and Prefix queries. It can only store String attribute values.
///
/// # Example
/// ```
/// use attribute_search_engine::{SearchIndex, SearchIndexPrefixTree};
/// use std::collections::HashSet;
/// use attribute_search_engine::Query;
///
/// let mut index_firstname = SearchIndexPrefixTree::<usize>::new();
/// index_firstname.insert(0, "Alex".into());
/// index_firstname.insert(1, "Alexander".into());
/// index_firstname.insert(2, "Andrea".into());
/// index_firstname.insert(3, "Ben".into());
///
/// let result = index_firstname.search(&Query::Exact("<unused>".into(), "Alex".into()));
/// assert_eq!(result, Ok(HashSet::from_iter(vec![0])));
///
/// let result = index_firstname.search(&Query::Prefix("<unused>".into(), "Alex".into()));
/// assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1])));
/// ```
pub struct SearchIndexPrefixTree<P> {
    index: HashSetPrefixTree<P>,
}

impl<P: Eq + Hash + Clone> Default for SearchIndexPrefixTree<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Eq + Hash + Clone> SearchIndexPrefixTree<P> {
    /// Creates a new `SearchIndexPrefixTree`.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::SearchIndexPrefixTree;
    ///
    /// let index = SearchIndexPrefixTree::<usize>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            index: HashSetPrefixTree::new(),
        }
    }

    /// Insert a new entry in the index.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::SearchIndexPrefixTree;
    ///
    /// let mut index = SearchIndexPrefixTree::<usize>::new();
    ///
    /// // You insert an entry by giving a row / primary id and an attribute value:
    /// index.insert(123, "Hello".into());
    /// // The same row / primary id can have multiple attributes assigned:
    /// index.insert(123, "World".into());
    /// // Add as much entries as you want for as many rows you want:
    /// index.insert(124, "Rust".into());
    /// ```
    pub fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index.insert(&attribute_value, primary_id);
    }
}

impl<P: Eq + Hash + Clone> SearchIndex<P> for SearchIndexPrefixTree<P> {
    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(_, value) => Ok(self.index.get(value).unwrap_or_default()),
            Query::Prefix(_, value) => Ok(self.index.get_prefix(value).unwrap_or_default()),
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_index_exact_string() {
        let mut index = SearchIndexPrefixTree::<usize>::new();
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
    fn search_index_prefix_string() {
        let mut index = SearchIndexPrefixTree::<usize>::new();
        index.insert(0, "A".into());
        index.insert(1, "AA".into());
        index.insert(2, "AB".into());
        index.insert(3, "ABA".into());
        index.insert(4, "ABB".into());
        index.insert(5, "ABC".into());
        index.insert(6, "B".into());
        index.insert(7, "BA".into());
        index.insert(8, "BB".into());

        let result = index.search(&Query::Prefix("<not used>".into(), "A".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3, 4, 5])));

        let result = index.search(&Query::Prefix("<not used>".into(), "B".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![6, 7, 8])));

        let result = index.search(&Query::Prefix("<not used>".into(), "C".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));

        let result = index.search(&Query::Prefix("<not used>".into(), "AB".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![2, 3, 4, 5])));

        let result = index.search(&Query::Prefix("<not used>".into(), "ABC".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![5])));
    }

    #[test]
    fn search_index_unsupported_queries() {
        let mut index = SearchIndexPrefixTree::<usize>::new();
        index.insert(0, "".into());

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
