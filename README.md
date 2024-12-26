# Attribute Search Engine

[<img alt="github" src="https://img.shields.io/badge/github-NHollmann/attribute--search--engine-77b0fc?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/NHollmann/attribute-search-engine)
[<img alt="crates.io" src="https://img.shields.io/crates/v/attribute-search-engine.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/attribute-search-engine)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-attribute--search--engine-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/attribute-search-engine)


Attribute Search Engine is a generic search engine for rows consisting of attributes, that can be searched using different matchers.

**Warning: This project is not finished yet and the public interface may change.**

- Rows
  - Attributes
    - ExactMatch  (HashMap)
    - PrefixMatch (PrefixTree/Trie)
    - RangeMatch  (BTreeMap)
- Queries
  - Are in CNF (Conjunctive Normal Form)
    Example: `+name:Hans,Peter +age:25-35 -lastname=Doe`
    Means:   `(name=Hans || name==Peter) && (age >= 25 && age <= 35) && !(lastname=Doe)`
  - 1. get a set for each predicate
    2. get the union of these sets for each disjunction
    3. get the intersections of the resulting sets for each conjunction
      if the result is empty at some point you can short-circuit and stop
    4. get the difference of the conjunction result with all negated conjunctions
      if the result is empty at some point you can short-circuit and stop
    5. optionally perform a fulltext search
