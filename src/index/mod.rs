use crate::{Query, Result, SearchEngineError};
use std::{collections::HashSet, str::FromStr};

mod hashmap;
mod prefix;
mod btree_range;

pub use hashmap::*;
pub use prefix::*;
pub use btree_range::*;

pub trait SearchIndex<P> {
    fn search(&self, query: &Query) -> Result<HashSet<P>>;
}

fn query_string_to_type<T: FromStr>(value: &str) -> Result<T> {
    value
        .parse()
        .map_err(|_| SearchEngineError::MismatchedQueryType)
}
