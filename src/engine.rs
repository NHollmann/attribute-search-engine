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
    ///
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

    pub fn query_from_str(&self, query_str: &str) -> Result<Query> {
        let mut include = vec![];
        let mut exclude = vec![];

        let lexer = QueryLexer::new(query_str);
        for subquery in lexer {
            match subquery {
                QueryToken::Attribute(is_include, attribute, values) => {
                    let mut qs: Vec<_> = values
                        .iter()
                        .map(|&v| Query::Exact(attribute.to_owned(), v.to_owned()))
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
                QueryToken::Freetext(_) => {}
            }
        }

        let base_query = Query::And(include);
        if !exclude.is_empty() {
            Ok(Query::Exclude(base_query.into(), exclude))
        } else {
            Ok(base_query)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_parser() {
        let engine = SearchEngine::<usize>::new();

        let q = engine.query_from_str("").unwrap();
        assert_eq!(q, Query::And(vec![]));

        let q = engine
            .query_from_str("+zipcode:12345 +pet:Dog -name:Hans")
            .unwrap();
        assert_eq!(
            q,
            Query::Exclude(
                Box::new(Query::And(vec![
                    Query::Exact("zipcode".into(), "12345".into()),
                    Query::Exact("pet".into(), "Dog".into())
                ])),
                vec![Query::Exact("name".into(), "Hans".into())]
            )
        );
    }
}
