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
//! let (q, _) = engine.query_from_str("+name:Alice +address:Germany/ -age:25").unwrap();
//! assert_eq!(engine.search(&q), Ok(HashSet::from_iter(vec![0])));
//! ```
//!
//! # [Search Indices](SearchIndex)
//! A SearchIndex saves the mapping from attribute values to primary IDs for a single attribute.
//! These indices can then be searched using queries. If only a single attribute type is needed,
//! a SearchIndex by itself may be enough. But normally they are added to a [SearchEngine] that
//! can handle multiple indices and complex queries involving `Or`, `And` and `Exclude` queries.
//!
//! This library provides the following types of search indices:
//! - [SearchIndexHashMap], backed by a HashMap for quick exact queries.
//! - [SearchIndexPrefixTree], backed by a prefix tree to find rows just by the prefix of an attribute.
//! - [SearchIndexBTreeRange], backed by a BTreeMap to find rows with an attribute by providing a range.
//!
//! The [SearchEngine] can also work with custom search indices as long as they implement the
//! [SearchIndex] trait.
//!
//! # [Queries](Query)
//! Queries are used to find rows in a [SearchIndex] or [SearchEngine]. [Query] is an enum type that defines
//! different search behaviours. Not all Query variants are supported by all index types. Queries can
//! be crafted manually without restrictions or with some limits from a string using a SearchEngine.
//!
//! The following table shows which Query variant is supported by which index type.
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
//! ## Query String Syntax
//!
//! The SearchEngine provides the function `query_from_str` that can be used to create queries
//! from strings. They are much more limited than manually crafted queries but should be
//! powerful enough for most use cases.
//!
//! The following text is an example for a query:
//! ```text
//! +attr1:foo,bar +attr2:=baz +attr3:<42,>84 -attr4:69-121
//! ```
//! As a boolean expression it will mean something like this:
//! ```text
//!    (attr1==foo || attr1==bar)
//! && (attr2==baz)
//! && (attr3 <= 42 || attr3 >= 84)
//! && !(69 <= attr4 <= 121)
//! ```
//!
//! Are more in-depth description of the query syntax can be found in the documentation of the
//! [SearchEngine::query_from_str] function.
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
