use std::{fmt, result};

/// Common Result type for the attribute search engine.
pub type Result<T> = result::Result<T, SearchEngineError>;

/// Enum of all possible error types that the attribute search engine
/// can throw by itself.
#[derive(Debug)]
pub enum SearchEngineError {
    /// Will be thrown if an unknown attribute is requested,
    /// for example when inserting or by a [Query](crate::query::Query).
    UnknownAttribute,

    /// A [Query](crate::query::Query) value cannot be processed by a
    /// specific search index because the string can't be converted to the expected type.
    MismatchedQueryType,

    /// A [Query](crate::query::Query) cannot be processed because it is
    /// not supported.
    UnsupportedQuery,
}

impl std::error::Error for SearchEngineError {}

impl fmt::Display for SearchEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchEngineError::UnknownAttribute => write!(f, "Unknown attribute error"),
            SearchEngineError::MismatchedQueryType => write!(f, "Mismatched query type"),
            SearchEngineError::UnsupportedQuery => write!(f, "Unsupported query"),
        }
    }
}
