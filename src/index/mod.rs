use crate::{Query, Result, SearchEngineError};
use std::{collections::HashSet, str::FromStr};

mod btree_range;
mod hashmap;
mod prefix;

pub use btree_range::*;
pub use hashmap::*;
pub use prefix::*;

pub trait SearchIndex<P> {
    fn search(&self, query: &Query) -> Result<HashSet<P>>;
}

fn query_string_to_type<T: FromStr>(value: &str) -> Result<T> {
    value
        .parse()
        .map_err(|_| SearchEngineError::MismatchedQueryType)
}
