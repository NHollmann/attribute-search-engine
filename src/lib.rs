//! Attribute Search Engine is a generic search engine for rows
//! consisting of attributes, that can be searched using different matchers.
//!
//! # Overview
//! ```rust
//! use attribute_search_engine::*;
//! use std::collections::HashSet;
//!
//! // Before we can create a new engine we need some indices.
//! let mut index_name = SearchIndexHashMap::<_, String>::new();
//! let mut index_age = SearchIndexBTreeRange::<_, u8>::new();
//! let mut index_address = SearchIndexPrefixTree::<_>::new();
//!
//! // We add two persons:
//! index_name.insert(0, "Alice".into());
//! index_age.insert(0, 27);
//! index_address.insert(0, "Germany/Hamburg".into());
//!
//! index_name.insert(1, "Bob".into());
//! index_name.insert(1, "Bobby".into()); // One row can have multiple entries
//! index_age.insert(1, 25);
//! index_address.insert(1, "Germany/Berlin".into());
//!
//! // Now we create the engine and add our indices to it:
//! let mut engine = SearchEngine::<usize>::new();
//! engine.add_index("name", index_name);
//! engine.add_index("age", index_age);
//! engine.add_index("address", index_address);
//! 
//! // We can create queries of any complexity with the Query type.
//! let q = Query::And(vec![
//!     Query::Or(vec![
//!         Query::Exact("name".into(), "Alice".into()),
//!         Query::Exact("name".into(), "Bob".into()),
//!     ]),
//!     Query::Prefix("address".into(), "Germany/".into()),
//! ]);
//! assert_eq!(engine.search(&q), Ok(HashSet::from_iter(vec![0, 1])));
//! 
//! // The search engine also has the ability to parse strings into
//! // queries. Check the documentation of SearchEngine::query_from_str
//! // for more details. Parsed queries are by design a lot more limited
//! // then manually constructed queries. The construction of queries
//! // can fail for example if unknown indices are referenced.
//! let q = engine.query_from_str("+name:Alice +address:Germany/Hamburg -age:25").unwrap();
//! assert_eq!(engine.search(&q), Ok(HashSet::from_iter(vec![0])));
//! ```
//!
//! # Which [Index](SearchIndex) should I use?
//!
//! | [Query]                     | [SearchIndexHashMap] | [SearchIndexPrefixTree] | [SearchIndexBTreeRange] |
//! |-----------------------------|----------------------|-------------------------|-------------------------|
//! | [Exact](Query::Exact)       | Yes ✔️               | Yes ✔️                 | Yes ✔️                 |
//! | [Prefix](Query::Prefix)     | No  ❌               | Yes ✔️                 | No  ❌                 |
//! | [InRange](Query::InRange)   | No  ❌               | No  ❌                 | Yes ✔️                 |
//! | [OutRange](Query::OutRange) | No  ❌               | No  ❌                 | Yes ✔️                 |
//! | [Minimum](Query::Minimum)   | No  ❌               | No  ❌                 | Yes ✔️                 |
//! | [Maximum](Query::Maximum)   | No  ❌               | No  ❌                 | Yes ✔️                 |
//! | [Or](Query::Or)             | na[^searchengine] 🔷 | na[^searchengine] 🔷   | na[^searchengine] 🔷   |
//! | [And](Query::And)           | na[^searchengine] 🔷 | na[^searchengine] 🔷   | na[^searchengine] 🔷   |
//! | [Exclude](Query::Exclude)   | na[^searchengine] 🔷 | na[^searchengine] 🔷   | na[^searchengine] 🔷   |
//!
//! [^searchengine]: Or, And & Exclude are only supported by [SearchEngine] and not
//!                  the indices.
//!

mod engine;
mod error;
mod index;
mod query;
mod query_lexer;

pub use engine::*;
pub use error::*;
pub use index::*;
pub use query::*;
