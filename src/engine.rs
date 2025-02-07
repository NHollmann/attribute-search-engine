use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::error::*;
use crate::index::*;
use crate::query::*;
use crate::query_lexer::*;

/// A SearchEngine is a wrapper around a collection of [search indices](SearchIndex)
/// that can process complex [queries](Query) involving multiple indices.
///
/// It can also create [queries](Query) from strings that are tailored to the
/// existing [indices](SearchIndex).
///
/// # Example
/// A complete example can be found on the [front page of this crate](crate).
pub struct SearchEngine<P> {
    indices: HashMap<String, Box<dyn SearchIndex<P>>>,
}

impl<P: Eq + Hash + Clone> Default for SearchEngine<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Eq + Hash + Clone> SearchEngine<P> {
    /// Creates a new `SearchEngine`.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::SearchEngine;
    ///
    /// let engine = SearchEngine::<usize>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
        }
    }

    /// Add a new index to this search engine.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::{SearchEngine, SearchIndexHashMap};
    ///
    /// let mut index = SearchIndexHashMap::<_, String>::new();
    /// // Fill index here...
    ///
    /// let mut engine = SearchEngine::<usize>::new();
    /// engine.add_index("attribute", index);
    /// ```
    pub fn add_index<T: SearchIndex<P> + 'static>(&mut self, name: &str, index: T) {
        self.indices.insert(name.into(), Box::new(index));
    }

    /// Run a query on the search engine.
    ///
    /// The result is a HashSet of all row ids / primary ids
    /// with rows that matched the query.
    pub fn search(&self, query: &Query) -> Result<HashSet<P>> {
        match query {
            Query::Exact(attr, _)
            | Query::Prefix(attr, _)
            | Query::InRange(attr, _, _)
            | Query::OutRange(attr, _, _)
            | Query::Minimum(attr, _)
            | Query::Maximum(attr, _) => {
                let index = self
                    .indices
                    .get(attr)
                    .ok_or(SearchEngineError::UnknownAttribute)?;
                index.search(query)
            }
            Query::Or(vec) => {
                let mut result_set = HashSet::<P>::new();
                for pred in vec.iter() {
                    let attribute_set = self.search(pred)?;
                    result_set = result_set.union(&attribute_set).cloned().collect();
                }
                Ok(result_set)
            }
            Query::And(vec) => {
                let mut result_set = HashSet::<P>::new();
                for (i, pred) in vec.iter().enumerate() {
                    let attribute_set = self.search(pred)?;
                    if i == 0 {
                        result_set = attribute_set;
                    } else {
                        result_set = result_set.intersection(&attribute_set).cloned().collect();
                    }
                    if result_set.is_empty() {
                        return Ok(result_set);
                    }
                }
                Ok(result_set)
            }
            Query::Exclude(base, exclude) => {
                let mut result_set = self.search(base)?;
                for pred in exclude.iter() {
                    let attribute_set = self.search(pred)?;
                    result_set = result_set.difference(&attribute_set).cloned().collect();
                    if result_set.is_empty() {
                        return Ok(result_set);
                    }
                }
                Ok(result_set)
            }
        }
    }

    /// Build a [Query] from a string slice.
    ///
    /// This function can return an error if an unknown index is referenced.
    /// On success this function returns a fully owned Query and a vector of string
    /// slices into the input query, that contains all parts of the query, that are not
    /// following the query syntax. They are called "Freetext". They will never contain
    /// whitespace.
    ///
    /// # Basic Syntax
    /// The following text is an example for a query:
    /// ```text
    /// +attr1:foo,bar +attr2:=baz +attr3:<42,>84 -attr4:69-121
    /// ```
    /// As a boolean expression it will mean something like this:
    /// ```text
    ///    (attr1==foo || attr1==bar)
    /// && (attr2==baz)
    /// && (attr3 <= 42 || attr3 >= 84)
    /// && !(69 <= attr4 <= 121)
    /// ```
    ///
    /// # In-depth Syntax description
    /// A string query consists of multiple whitespace seperated attribute selectors.
    /// Each of them starts with a `+` or `-` sign, indicating if the rows matching this
    /// selector should be included or excluded from the result. This is followed by the
    /// name of the attribute/index and a single `:` char. Next comes a list of comma seperated
    /// values that describe the basic queries that are used to select rows. There are
    /// special operator symbols that can change the meaning of a value if the index
    /// supports the matching query type. For example, if the Index supports Maximum queries,
    /// the following value will return a Maximum query instead of an Exact query: `<123`.
    ///
    /// The following operator symbols are currently used **if the index supports it**:
    /// - `>val` - forces a Minimum query
    /// - `<val` - forces a Maximum query
    /// - `=val` - forces a Exact query
    /// - `minval-maxval` - forces a InRange query
    ///
    /// If no operator symbol is found, a Prefix query will be used if it is supported by the index.
    /// Otherwise a Exact query is used, even if the index may not support it (all official indices
    /// currently implement them).
    ///
    /// All non-whitespace substrings in the query, that are not valid attribute selectors are
    /// considered "Freetext". All of these are returned on success and can be used or
    /// ignored by the caller. For example they can be used to filter the results further.
    ///
    /// # Limits
    /// - OutRange queries don't have an operator symbol and are currently not supported.
    ///   But it is possible to build a functionally equivalent query if the index supports
    ///   Minimum and Maximum queries: `+attr:<10,>20`
    /// - InRange does not support negative values because only one `-` char is allowed.
    /// - There is no way to force a Prefix query. It will be automatically used if no
    ///   operator symbol is found and the index supports them.
    ///
    /// # Example
    /// ```rust
    /// use attribute_search_engine::{SearchEngine, SearchIndexHashMap, Query};
    ///
    /// let mut index = SearchIndexHashMap::<_, String>::new();
    /// // Fill index here...
    ///
    /// let mut engine = SearchEngine::<usize>::new();
    /// engine.add_index("attribute", index);
    /// let (q, freetext) = engine.query_from_str("+attribute:foo bar").expect("no error");
    /// assert_eq!(q, Query::And(vec![Query::Exact("attribute".into(), "foo".into())]));
    /// assert_eq!(freetext, vec!["bar"]);
    /// ```
    pub fn query_from_str<'a>(&self, query_str: &'a str) -> Result<(Query, Vec<&'a str>)> {
        let mut include = vec![];
        let mut exclude = vec![];
        let mut freetexts = vec![];

        let lexer = QueryLexer::new(query_str);
        for subquery in lexer {
            match subquery {
                QueryToken::Attribute(is_include, attribute, values) => {
                    let index = self
                        .indices
                        .get(attribute)
                        .ok_or(SearchEngineError::UnknownAttribute)?;
                    let supported = index.supported_queries();

                    let mut qs: Vec<_> = values
                        .iter()
                        .map(|&v| {
                            let attr = attribute.to_owned();
                            if (supported & SUPPORTS_MINIMUM) != 0 && v.starts_with('>') {
                                return Query::Minimum(attr, v[1..].to_owned());
                            }
                            if (supported & SUPPORTS_MAXIMUM) != 0 && v.starts_with('<') {
                                return Query::Maximum(attr, v[1..].to_owned());
                            }
                            if (supported & SUPPORTS_EXACT) != 0 && v.starts_with('=') {
                                return Query::Exact(attr, v[1..].to_owned());
                            }
                            if (supported & SUPPORTS_INRANGE) != 0 && v.contains('-') {
                                let parts = v.split('-').collect::<Vec<_>>();
                                if parts.len() == 2 {
                                    return Query::InRange(
                                        attr,
                                        parts[0].to_owned(),
                                        parts[1].to_owned(),
                                    );
                                }
                            }

                            // Fallback, if nothing is found we use prefix if we can
                            // and exact otherwise.
                            if (supported & SUPPORTS_PREFIX) != 0 {
                                return Query::Prefix(attr, v.to_owned());
                            }
                            Query::Exact(attr, v.to_owned())
                        })
                        .collect();
                    let q = match qs.len().cmp(&1) {
                        Ordering::Equal => qs.swap_remove(0),
                        Ordering::Greater => Query::Or(qs),
                        Ordering::Less => continue,
                    };
                    if is_include {
                        include.push(q);
                    } else {
                        exclude.push(q);
                    }
                }
                QueryToken::Freetext(text) => {
                    freetexts.push(text);
                }
            }
        }

        let base_query = Query::And(include);
        if !exclude.is_empty() {
            Ok((Query::Exclude(base_query.into(), exclude), freetexts))
        } else {
            Ok((base_query, freetexts))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyIndex {
        fixed_values: HashSet<usize>,
        supported_queries: SupportedQueries,
    }

    impl DummyIndex {
        fn new(vals: Vec<usize>) -> Self {
            Self {
                fixed_values: HashSet::from_iter(vals),
                supported_queries: SUPPORTS_EXACT,
            }
        }

        fn supports(sup: SupportedQueries) -> Self {
            Self {
                fixed_values: HashSet::new(),
                supported_queries: sup,
            }
        }
    }

    impl SearchIndex<usize> for DummyIndex {
        fn search(&self, _query: &Query) -> Result<HashSet<usize>> {
            Ok(self.fixed_values.clone())
        }

        fn supported_queries(&self) -> SupportedQueries {
            self.supported_queries
        }
    }

    #[test]
    fn search_or() {
        let mut engine = SearchEngine::<usize>::new();
        engine.add_index("a", DummyIndex::new(vec![1, 2]));
        engine.add_index("b", DummyIndex::new(vec![3, 4]));
        engine.add_index("c", DummyIndex::new(vec![2, 5, 6]));
        let result = engine.search(&Query::Or(vec![
            Query::Exact("a".into(), "DUMMY".into()),
            Query::Exact("c".into(), "DUMMY".into()),
        ]));
        assert_eq!(result, Ok(HashSet::from_iter(vec![1, 2, 5, 6])));
    }

    #[test]
    fn search_and() {
        let mut engine = SearchEngine::<usize>::new();
        engine.add_index("a", DummyIndex::new(vec![1, 2]));
        engine.add_index("b", DummyIndex::new(vec![3, 4]));
        engine.add_index("c", DummyIndex::new(vec![2, 5, 6]));
        let result = engine.search(&Query::And(vec![
            Query::Exact("a".into(), "DUMMY".into()),
            Query::Exact("c".into(), "DUMMY".into()),
        ]));
        assert_eq!(result, Ok(HashSet::from_iter(vec![2])));
    }

    #[test]
    fn search_exclude() {
        let mut engine = SearchEngine::<usize>::new();
        engine.add_index("a", DummyIndex::new(vec![1, 2]));
        engine.add_index("b", DummyIndex::new(vec![3, 4]));
        engine.add_index("c", DummyIndex::new(vec![2, 5, 6]));
        let result = engine.search(&Query::Exclude(
            Box::new(Query::Exact("c".into(), "DUMMY".into())),
            vec![Query::Exact("a".into(), "DUMMY".into())],
        ));
        assert_eq!(result, Ok(HashSet::from_iter(vec![5, 6])));
    }

    fn create_parser_engine() -> SearchEngine<usize> {
        let mut engine = SearchEngine::<usize>::new();
        engine.add_index(
            "zipcode",
            DummyIndex::supports(
                SUPPORTS_EXACT | SUPPORTS_MINIMUM | SUPPORTS_MAXIMUM | SUPPORTS_INRANGE,
            ),
        );
        engine.add_index("pet", DummyIndex::supports(SUPPORTS_EXACT));
        engine.add_index(
            "name",
            DummyIndex::supports(SUPPORTS_EXACT | SUPPORTS_PREFIX),
        );
        engine
    }

    #[test]
    fn query_parser_empty() {
        let engine = create_parser_engine();
        let (q, freetext) = engine.query_from_str("").unwrap();
        assert_eq!(q, Query::And(vec![]));
        assert_eq!(freetext, vec![] as Vec<&str>);
    }

    #[test]
    fn query_parser_basic() {
        let engine = create_parser_engine();
        let (q, freetext) = engine
            .query_from_str("+zipcode:12345 +pet:Dog -name:Hans freetext")
            .unwrap();
        assert_eq!(
            q,
            Query::Exclude(
                Box::new(Query::And(vec![
                    Query::Exact("zipcode".into(), "12345".into()),
                    Query::Exact("pet".into(), "Dog".into())
                ])),
                vec![Query::Prefix("name".into(), "Hans".into())]
            )
        );
        assert_eq!(freetext, vec!["freetext"]);
    }

    #[test]
    fn query_parser_modificators() {
        let engine = create_parser_engine();
        let (q, freetext) = engine
            .query_from_str(
                "abc +zipcode:>12345 +zipcode:<99999 +zipcode:50000-60000 +name:=Hans def",
            )
            .unwrap();
        assert_eq!(
            q,
            Query::And(vec![
                Query::Minimum("zipcode".into(), "12345".into()),
                Query::Maximum("zipcode".into(), "99999".into()),
                Query::InRange("zipcode".into(), "50000".into(), "60000".into()),
                Query::Exact("name".into(), "Hans".into()),
            ])
        );
        assert_eq!(freetext, vec!["abc", "def"]);
    }

    #[test]
    fn query_parser_alternatives() {
        let engine = create_parser_engine();
        let (q, freetext) = engine
            .query_from_str(
                "start +pet:Cat,Dog +name:Alex,=Hans middle +zipcode:=10000,<500,50000-60000 end",
            )
            .unwrap();
        assert_eq!(
            q,
            Query::And(vec![
                Query::Or(vec![
                    Query::Exact("pet".into(), "Cat".into()),
                    Query::Exact("pet".into(), "Dog".into()),
                ]),
                Query::Or(vec![
                    Query::Prefix("name".into(), "Alex".into()),
                    Query::Exact("name".into(), "Hans".into()),
                ]),
                Query::Or(vec![
                    Query::Exact("zipcode".into(), "10000".into()),
                    Query::Maximum("zipcode".into(), "500".into()),
                    Query::InRange("zipcode".into(), "50000".into(), "60000".into()),
                ]),
            ])
        );
        assert_eq!(freetext, vec!["start", "middle", "end"]);
    }
}
