use std::{fmt, result};

pub type Result<T> = result::Result<T, SearchEngineError>;

#[derive(Debug)]
pub enum SearchEngineError {
    UnknownArgument,
    MismatchedQueryType,
}

impl std::error::Error for SearchEngineError {}

impl fmt::Display for SearchEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchEngineError::UnknownArgument => write!(f, "Unknown argument error"),
            SearchEngineError::MismatchedQueryType => write!(f, "Mismatched query type"),
        }
    }
}
