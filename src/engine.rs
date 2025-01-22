use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::attributes::*;
use crate::error::*;
use crate::index::*;
use crate::query::*;

pub struct SearchEngine<P: Eq + Hash + Clone + 'static> {
    indices: HashMap<String, Box<dyn SearchIndex<P>>>,
}

impl<P: Eq + Hash + Clone + 'static> SearchEngine<P> {
    pub fn new(schema: &AttributeSchema) -> Self {
        let mut indices = HashMap::with_capacity(schema.count());

        for (name, t) in schema.iter() {
            match t {
                AttributeKind::ExactMatch => {
                    indices.insert(name.clone(), Box::new(SearchIndexExact::<P>::new()) as _);
                }
                AttributeKind::PrefixMatch => {
                    indices.insert(name.clone(), Box::new(SearchIndexPrefix::<P>::new()) as _);
                }
                AttributeKind::RangeMatch => {
                    indices.insert(name.clone(), Box::new(SearchIndexRange::<P>::new()) as _);
                }
            }
        }

        Self { indices }
    }

    pub fn insert(&mut self, primary_id: P, attribute: &str, attribute_value: &str) -> Result<()> {
        let index = self
            .indices
            .get_mut(attribute)
            .ok_or(SearchEngineError::UnknownArgument)?;
        index.insert(primary_id, attribute_value.to_string());
        Ok(())
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
                    .ok_or(SearchEngineError::UnknownArgument)?;
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
                    if result_set.len() == 0 {
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
                    if result_set.len() == 0 {
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
        if exclude.len() > 0 {
            Ok(Query::Exclude(base_query.into(), exclude))
        } else {
            Ok(base_query)
        }
    }
}
