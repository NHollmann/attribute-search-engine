# Attribute Search Engine

[<img alt="github" src="https://img.shields.io/badge/github-NHollmann/attribute--search--engine-77b0fc?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/NHollmann/attribute-search-engine)
[<img alt="crates.io" src="https://img.shields.io/crates/v/attribute-search-engine.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/attribute-search-engine)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-attribute--search--engine-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/attribute-search-engine)

Attribute Search Engine is a generic search engine for rows consisting of attributes, that can be searched using different matchers.
It also includes a query string parser for simple queries.

**Warning: This project is not stable yet and the public interface may change in the future. Especially the query syntax might change.**

- Rows
  - Attributes
    - ExactMatch  (HashMap)
    - PrefixMatch (PrefixTree/Trie) (Strings only)
    - RangeMatch  (BTreeMap)
- Queries
  - Are in CNF (Conjunctive Normal Form)
    Example: `+name:Hans,Peter +age:25-35 -lastname=Doe`
    Means:   `(name=Hans || name==Peter) && (age >= 25 && age <= 35) && !(lastname=Doe)`

# Overview
```rust
use attribute_search_engine::*;
use std::collections::HashSet;

// Before we can create a new engine we need some indices.
let mut index_name = SearchIndexHashMap::<_, String>::new();
let mut index_age = SearchIndexBTreeRange::<_, u8>::new();
let mut index_address = SearchIndexPrefixTree::<_>::new();

// We add two persons:
index_name.insert(0, "Alice".into());
index_age.insert(0, 27);
index_address.insert(0, "Germany/Hamburg".into());

index_name.insert(1, "Bob".into());
index_name.insert(1, "Bobby".into()); // One row can have multiple entries
index_age.insert(1, 25);
index_address.insert(1, "Germany/Berlin".into());

// Now we create the engine and add our indices to it:
let mut engine = SearchEngine::<usize>::new();
engine.add_index("name", index_name);
engine.add_index("age", index_age);
engine.add_index("address", index_address);

// We can create queries of any complexity with the Query type.
let q = Query::And(vec![
    Query::Or(vec![
        Query::Exact("name".into(), "Alice".into()),
        Query::Exact("name".into(), "Bob".into()),
    ]),
    Query::Prefix("address".into(), "Germany/".into()),
]);
assert_eq!(engine.search(&q), Ok(HashSet::from_iter(vec![0, 1])));

// The search engine also has the ability to parse strings into
// queries. Check the documentation of SearchEngine::query_from_str
// for more details. Parsed queries are by design a lot more limited
// then manually constructed queries. The construction of queries
// can fail for example if unknown indices are referenced.
let q = engine.query_from_str("+name:Alice +address:Germany/ -age:25").unwrap();
assert_eq!(engine.search(&q), Ok(HashSet::from_iter(vec![0])));
```
