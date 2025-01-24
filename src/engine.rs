use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::error::*;
use crate::index::*;
use crate::query::*;

pub struct SearchEngine<P> {
    indices: HashMap<String, Box<dyn SearchIndex<P>>>,
}

impl<P: Eq + Hash + Clone> Default for SearchEngine<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Eq + Hash + Clone> SearchEngine<P> {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
        }
    }

    pub fn add_index<T: SearchIndex<P> + 'static>(&mut self, name: &str, index_builder: T) {
        self.indices.insert(name.into(), Box::new(index_builder));
    }

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
        let attr_re = Regex::new(r"(\+|-)(\w+):(\S*)").expect("the regex to compile");

        let mut include = vec![];
        let mut exclude = vec![];
        for (_, [modifier, attribute, value]) in
            attr_re.captures_iter(query_str).map(|c| c.extract())
        {
            let new_predicate = Query::Exact(attribute.to_owned(), value.to_owned());
            if modifier == "+" {
                include.push(new_predicate);
            } else {
                exclude.push(new_predicate);
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
