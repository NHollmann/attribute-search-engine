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

    pub fn search_attribute(
        &self,
        attribute: &str,
        attribute_value: &str,
    ) -> Result<HashSet<usize>> {
        let index = self
            .indices
            .get(attribute)
            .ok_or(SearchEngineError::UnknownArgument)?;
        Ok(index.search(attribute_value))
    }

    pub fn search(&self, query: &Query) -> Result<HashSet<usize>> {
        match query {
            Query::ExactString(attr, QueryValue::Str(val)) => self.search_attribute(attr, val),
            Query::ExactString(_attr, _val) => Err(SearchEngineError::MismatchedQueryType),
            Query::PrefixString(_, _) => todo!(),
            Query::InRange(_, _, _) => todo!(),
            Query::OutRange(_, _, _) => todo!(),
            Query::Minimum(_, _) => todo!(),
            Query::Maximum(_, _) => todo!(),
            Query::Or(vec) => {
                let mut result_set = HashSet::<usize>::new();
                for pred in vec.iter() {
                    let attribute_set = self.search(pred)?;
                    result_set = result_set.union(&attribute_set).copied().collect();
                }
                Ok(result_set)
            },
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
            },
        }
    }
}
