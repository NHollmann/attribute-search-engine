use super::{string_to_payload_type, SearchIndex};
use crate::{
    Query, Result, SearchEngineError, SupportedQueries, SUPPORTS_EXACT, SUPPORTS_INRANGE,
    SUPPORTS_MAXIMUM, SUPPORTS_MINIMUM, SUPPORTS_OUTRANGE,
};
use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

/// SearchIndexBTreeRange is a index backed by a BTreeMap that can match
/// Exact, InRange, OutRange, Minimum and Maximum queries.
///
/// # Example
/// ```
/// use attribute_search_engine::{SearchIndex, SearchIndexBTreeRange};
/// use std::collections::HashSet;
/// use attribute_search_engine::Query;
///
/// let mut index_age = SearchIndexBTreeRange::<usize, i32>::new();
/// index_age.insert(0, 17);
/// index_age.insert(1, 42);
/// index_age.insert(2, 31);
/// index_age.insert(3, 26);
///
/// let result = index_age.search(&Query::Exact("<unused>".into(), "42".into()));
/// assert_eq!(result, Ok(HashSet::from_iter(vec![1])));
///
/// let result = index_age.search(&Query::InRange("<unused>".into(), "20".into(), "40".into()));
/// assert_eq!(result, Ok(HashSet::from_iter(vec![2, 3])));
/// ```
pub struct SearchIndexBTreeRange<P, V> {
    index: BTreeMap<V, HashSet<P>>,
}

impl<P, V> Default for SearchIndexBTreeRange<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Ord + FromStr + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P, V> SearchIndexBTreeRange<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Ord + FromStr + 'static,
{
    /// Creates a new `SearchIndexBTreeRange`.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::SearchIndexBTreeRange;
    ///
    /// let index = SearchIndexBTreeRange::<usize, i32>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
        }
    }

    /// Insert a new entry in the index.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::SearchIndexBTreeRange;
    ///
    /// let mut index = SearchIndexBTreeRange::<usize, i32>::new();
    ///
    /// // You insert an entry by giving a row / primary id and an attribute value:
    /// index.insert(123, 42);
    /// // The same row / primary id can have multiple attributes assigned:
    /// index.insert(123, 69);
    /// // Add as much entries as you want for as many rows you want:
    /// index.insert(124, 32);
    /// ```
    pub fn insert(&mut self, primary_id: P, attribute_value: V) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    /// This internal function helps with searching for all kinds of
    /// ranges and merging the result to a HashSet.
    fn search_range(&self, range: impl RangeBounds<V>) -> HashSet<P> {
        let mut result_set = HashSet::<P>::new();
        for (_, primary_set) in self.index.range(range) {
            result_set = result_set.union(primary_set).cloned().collect();
        }
        result_set
    }
}

impl<P, V> SearchIndex<P> for SearchIndexBTreeRange<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Ord + FromStr + 'static,
{
    fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(_, value_str) => {
                let value: V = string_to_payload_type(value_str)?;
                Ok(self.index.get(&value).cloned().unwrap_or(HashSet::new()))
            }
            Query::InRange(_, min_str, max_str) => {
                let min: V = string_to_payload_type(min_str)?;
                let max: V = string_to_payload_type(max_str)?;
                if min > max {
                    return Ok(HashSet::new());
                }
                Ok(self.search_range(min..=max))
            }
            Query::Minimum(_, min_str) => {
                let min: V = string_to_payload_type(min_str)?;
                Ok(self.search_range(min..))
            }
            Query::Maximum(_, max_str) => {
                let max: V = string_to_payload_type(max_str)?;
                Ok(self.search_range(..=max))
            }
            Query::OutRange(_, start_str, end_str) => {
                let start: V = string_to_payload_type(start_str)?;
                let end: V = string_to_payload_type(end_str)?;
                if start > end {
                    return Ok(HashSet::new());
                }
                Ok(self
                    .search_range(..start)
                    .union(&self.search_range((Bound::Excluded(end), Bound::Unbounded)))
                    .cloned()
                    .collect())
            }
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }

    fn supported_queries(&self) -> SupportedQueries {
        SUPPORTS_EXACT | SUPPORTS_INRANGE | SUPPORTS_MINIMUM | SUPPORTS_MAXIMUM | SUPPORTS_OUTRANGE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_index_exact_string() {
        let mut index = SearchIndexBTreeRange::<usize, String>::new();
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
        let mut index = SearchIndexBTreeRange::<usize, i32>::new();
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
    fn search_index_inrange_number() {
        let mut index = SearchIndexBTreeRange::<usize, i32>::new();
        index.insert(0, 00);
        index.insert(1, 10);
        index.insert(2, 20);
        index.insert(3, 30);
        index.insert(4, 40);
        index.insert(5, 50);

        let result = index.search(&Query::InRange(
            "<not used>".into(),
            "0".into(),
            "50".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3, 4, 5])));

        let result = index.search(&Query::InRange(
            "<not used>".into(),
            "10".into(),
            "40".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![1, 2, 3, 4])));

        let result = index.search(&Query::InRange(
            "<not used>".into(),
            "-20".into(),
            "20".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2])));

        let result = index.search(&Query::InRange(
            "<not used>".into(),
            "10".into(),
            "10".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![1])));

        let result = index.search(&Query::InRange(
            "<not used>".into(),
            "50".into(),
            "10".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));

        let result = index.search(&Query::InRange(
            "<not used>".into(),
            "51".into(),
            "100".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));
    }

    #[test]
    fn search_index_outrange_number() {
        let mut index = SearchIndexBTreeRange::<usize, i32>::new();
        index.insert(0, 00);
        index.insert(1, 10);
        index.insert(2, 20);
        index.insert(3, 30);
        index.insert(4, 40);
        index.insert(5, 50);

        let result = index.search(&Query::OutRange(
            "<not used>".into(),
            "0".into(),
            "50".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));

        let result = index.search(&Query::OutRange(
            "<not used>".into(),
            "10".into(),
            "40".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 5])));

        let result = index.search(&Query::OutRange(
            "<not used>".into(),
            "-20".into(),
            "20".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![3, 4, 5])));

        let result = index.search(&Query::OutRange(
            "<not used>".into(),
            "10".into(),
            "10".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 2, 3, 4, 5])));

        let result = index.search(&Query::OutRange(
            "<not used>".into(),
            "50".into(),
            "10".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));

        let result = index.search(&Query::OutRange(
            "<not used>".into(),
            "51".into(),
            "100".into(),
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3, 4, 5])));
    }

    #[test]
    fn search_index_minimum_number() {
        let mut index = SearchIndexBTreeRange::<usize, i32>::new();
        index.insert(0, 00);
        index.insert(1, 10);
        index.insert(2, 20);
        index.insert(3, 30);
        index.insert(4, 40);
        index.insert(5, 50);

        let result = index.search(&Query::Minimum("<not used>".into(), "0".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3, 4, 5])));

        let result = index.search(&Query::Minimum("<not used>".into(), "10".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![1, 2, 3, 4, 5])));

        let result = index.search(&Query::Minimum("<not used>".into(), "-20".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3, 4, 5])));

        let result = index.search(&Query::Minimum("<not used>".into(), "30".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![3, 4, 5])));

        let result = index.search(&Query::Minimum("<not used>".into(), "50".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![5])));

        let result = index.search(&Query::Minimum("<not used>".into(), "51".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));
    }

    #[test]
    fn search_index_maximum_number() {
        let mut index = SearchIndexBTreeRange::<usize, i32>::new();
        index.insert(0, 00);
        index.insert(1, 10);
        index.insert(2, 20);
        index.insert(3, 30);
        index.insert(4, 40);
        index.insert(5, 50);

        let result = index.search(&Query::Maximum("<not used>".into(), "50".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3, 4, 5])));

        let result = index.search(&Query::Maximum("<not used>".into(), "40".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3, 4])));

        let result = index.search(&Query::Maximum("<not used>".into(), "30".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0, 1, 2, 3])));

        let result = index.search(&Query::Maximum("<not used>".into(), "0".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![0])));

        let result = index.search(&Query::Maximum("<not used>".into(), "-1".into()));
        assert_eq!(result, Ok(HashSet::from_iter(vec![])));
    }

    #[test]
    fn search_index_unsupported_queries() {
        let mut index = SearchIndexBTreeRange::<usize, i32>::new();
        index.insert(0, 0);

        assert_eq!(
            index.search(&Query::Prefix("<not used>".into(), "0".into())),
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
