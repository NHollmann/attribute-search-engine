# Attribute Search Engine

[<img alt="github" src="https://img.shields.io/badge/github-NHollmann/attribute--search--engine-77b0fc?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/NHollmann/attribute-search-engine)
[<img alt="crates.io" src="https://img.shields.io/crates/v/attribute-search-engine.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/attribute-search-engine)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-attribute--search--engine-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/attribute-search-engine)

Attribute Search Engine is a generic search engine for rows consisting of attributes, that can be searched using different matchers.

**Warning: This project is not finished yet and the public interface may change.**

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
use attribute_search_engine::{SearchEngine, SearchIndexHashMap, SearchIndexBTreeRange};

// Before we can create a new engine we need some indices.
let mut index_name = SearchIndexHashMap::<_, String>::new();
let mut index_age = SearchIndexBTreeRange::<_, u8>::new();

// We add two persons:
index_name.insert(0, "Alice".into());
index_age.insert(0, 27);

index_name.insert(1, "Bob".into());
index_name.insert(1, "Bobby".into()); // One row can have multiple entries
index_age.insert(1, 25);

// Now we create the engine and add our indices to it:
let mut engine = SearchEngine::<usize>::new();
engine.add_index("name", index_name);
engine.add_index("age", index_age);

// TODO Manual Queries, Queries out of strings, Prefix-Index
```
