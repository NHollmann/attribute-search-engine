use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::attributes::*;
use crate::error::*;
use crate::index::*;
use crate::query::*;

pub struct SearchEngine {
    indices: HashMap<String, Box<dyn SearchIndex<usize>>>,
}

impl SearchEngine {
    pub fn new(schema: &AttributeSchema) -> Self {
        let mut indices = HashMap::with_capacity(schema.count());

        for (name, t) in schema.iter() {
            match t {
                AttributeKind::ExactMatch => {
                    indices.insert(
                        name.clone(),
                        Box::new(SearchIndexExact::<usize>::new()) as _,
                    );
                }
                AttributeKind::PrefixMatch => {
                    indices.insert(
                        name.clone(),
                        Box::new(SearchIndexPrefix::<usize>::new()) as _,
                    );
                }
                AttributeKind::RangeMatch => {
                    indices.insert(
                        name.clone(),
                        Box::new(SearchIndexRange::<usize>::new()) as _,
                    );
                }
            }
        }

        Self { indices }
    }

    pub fn insert(&mut self, primary_id: usize, attribute: &str, attribute_value: &str) {
        match self.indices.get_mut(attribute) {
            Some(index) => index.insert(primary_id, attribute_value.to_string()),
            _ => {}
        }
    }

    pub fn search(&self, query: &Query) -> Result<HashSet<usize>> {
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
                let mut result_set = HashSet::<usize>::new();
                for pred in vec.iter() {
                    let attribute_set = self.search(pred)?;
                    result_set = result_set.union(&attribute_set).copied().collect();
                }
                Ok(result_set)
            }
            Query::And(vec) => {
                let mut result_set = HashSet::<usize>::new();
                for (i, pred) in vec.iter().enumerate() {
                    let attribute_set = self.search(pred)?;
                    if i == 0 {
                        result_set = attribute_set;
                    } else {
                        result_set = result_set.intersection(&attribute_set).copied().collect();
                    }
                    if result_set.len() == 0 {
                        return Ok(result_set);
                    }
                }
                Ok(result_set)
            }
            Query::Exclude(vec) => {
                let mut result_set = HashSet::<usize>::new();
                for (i, pred) in vec.iter().enumerate() {
                    let attribute_set = self.search(pred)?;
                    if i == 0 {
                        result_set = attribute_set;
                    } else {
                        result_set = result_set.difference(&attribute_set).copied().collect();
                    }
                    if result_set.len() == 0 {
                        return Ok(result_set);
                    }
                }
                Ok(result_set)
            }
        }
    }

    pub fn query_from_str(query_str: &str) -> Result<Query> {
        // TODO: Support numbers, comma sperators (OR) and minus symbols (RANGES)
        let re = Regex::new(r"([\+-])(\w):(\w)").expect("the regex to compile");
        let mut results = vec![];
        for (_, [modifiery, attribute, value]) in re.captures_iter(query_str).map(|c| c.extract()) {
            results.push((modifiery, attribute, value));
        }
        // TODO: Transform captures to a query
        Ok(Query::Exact(
            query_str.into(),
            QueryValue::Str(query_str.into()),
        ))

        // // Err(SearchEngineError::InvalidQuery)
    }
}
