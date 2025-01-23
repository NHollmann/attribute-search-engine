use super::{string_to_payload_type, SearchIndex};
use crate::{Query, Result, SearchEngineError};
use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

pub struct SearchIndexBTreeRange<P, V> {
    index: BTreeMap<V, HashSet<P>>,
}

impl<P, V> SearchIndexBTreeRange<P, V>
where
    P: Eq + Hash + Clone + 'static,
    V: Ord + FromStr + 'static,
{
    /// Create a new SearchIndexBTreeRange.
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
        }
    }

    fn search_range(&self, range: impl RangeBounds<V>) -> HashSet<P> {
        let mut result_set = HashSet::<P>::new();
        for (_, ref primary_set) in self.index.range(range) {
            result_set = result_set.union(&primary_set).cloned().collect();
        }
        result_set
    }

    pub fn insert(&mut self, primary_id: P, attribute_value: V) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
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
                Ok(self
                    .index
                    .get(&value)
                    .cloned()
                    .unwrap_or(HashSet::<P>::new()))
            }
            Query::InRange(_, min_str, max_str) => {
                let min: V = string_to_payload_type(min_str)?;
                let max: V = string_to_payload_type(max_str)?;
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
                Ok(self
                    .search_range(..start)
                    .union(&self.search_range((Bound::Excluded(end), Bound::Unbounded)))
                    .cloned()
                    .collect())
            }
            _ => Err(SearchEngineError::UnsupportedQuery),
        }
    }
}
