use crate::{Query, Result, SearchEngineError, SupportedQueries};
use std::{collections::HashSet, str::FromStr};

mod btree_range;
mod hashmap;
mod prefix;

pub use btree_range::*;
pub use hashmap::*;
pub use prefix::*;

/// This trait describes the minimum features an Index must support to be
/// usable as a SearchIndex, for example in a [SearchEngine](crate::engine::SearchEngine).
pub trait SearchIndex<P> {
    /// Perform a search on an index.
    ///
    /// This function returns an HashSet of all matching results.
    /// It may not accept all enum values of [Query] but only
    /// a small subset. If a [Query] is not supported,
    /// this function returns [UnsupportedQuery](crate::error::SearchEngineError::UnsupportedQuery).
    ///
    /// If the strings in the [Query] cannot be parsed to
    /// the expected payload type, this function returns
    /// [MismatchedQueryType](crate::error::SearchEngineError::MismatchedQueryType).
    fn search(&self, query: &Query) -> Result<HashSet<P>>;

    /// Returns which queries are directly supported by an index.
    ///
    /// This function may be used for some features and optimizations by
    /// a [SearchEngine](crate::engine::SearchEngine). For example, it
    /// signals which operators (=,>,<,-) in the query parser are supported
    /// by an index.
    fn supported_queries(&self) -> SupportedQueries;
}

/// Tries to parse a string into a payload value.
///
/// This is an internal function. If it fails it returns
/// [MismatchedQueryType](crate::error::SearchEngineError::MismatchedQueryType)
/// to signal failure.
fn string_to_payload_type<T: FromStr>(value: &str) -> Result<T> {
    value
        .parse()
        .map_err(|_| SearchEngineError::MismatchedQueryType)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_payload_type() {
        assert_eq!(string_to_payload_type("1234"), Ok(1234i32));
        assert_eq!(string_to_payload_type("123456"), Ok(123456usize));
        assert_eq!(string_to_payload_type("true"), Ok(true));
        assert_eq!(
            string_to_payload_type::<usize>("Hello"),
            Err(SearchEngineError::MismatchedQueryType)
        );
    }
}
