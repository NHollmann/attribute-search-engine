# attribute-search-engine
Generic search engine for rows with attributes using different matchers.

- Rows
  - Attributes
    - ExactMatch  (HashMap)
    - PrefixMatch (PrefixTree/Trie)
    - RangeMatch  (BTreeMap)
- Queries
  - Are in CNF (Conjunctive Normal Form)
    Example: +name:Hans,Peter +age:25-35 -lastname=Doe
    Means:   (name=Hans || name==Peter) && (age >= 25 && age <= 35) && !(lastname=Doe)
  - 1. get a set for each predicate
    2. get the union of these sets for each disjunction
    3. get the intersections of the resulting sets for each conjunction
      if the result is empty at some point you can short-circuit and stop
    4. get the difference of the conjunction result with all negated conjunctions
      if the result is empty at some point you can short-circuit and stop
    5. optionally perform a fulltext search
