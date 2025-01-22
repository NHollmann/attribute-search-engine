use crate::{Query, Result};
use std::collections::HashSet;

mod exact;
//mod prefix;
//mod range;

pub use exact::*;
//pub use prefix::*;
//pub use range::*;

pub trait SearchIndex<P> {
    fn search(&self, query: &Query) -> Result<HashSet<P>>;
}

pub trait SearchIndexBuilder<P, V> {
    fn insert(&mut self, primary_id: P, attribute_value: V);
    fn build(self) -> Box<dyn SearchIndex<P>>;
}
