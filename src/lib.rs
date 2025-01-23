//! Attribute Search Engine is a generic search engine for rows
//! consisting of attributes, that can be searched using different matchers.

mod engine;
mod error;
mod index;
mod query;

pub use engine::*;
pub use error::*;
pub use index::*;
pub use query::*;
