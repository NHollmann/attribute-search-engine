use crate::{Query, Result};
use std::{collections::HashSet, hash::Hash};

mod exact;
mod prefix;
mod range;

pub use exact::*;
pub use prefix::*;
pub use range::*;

pub trait SearchIndex<P: Eq + Hash + Clone> {
    fn insert(&mut self, primary_id: P, attribute_value: String);
    fn search(&self, query: &Query) -> Result<HashSet<P>>;
}
