use std::collections::{HashMap, HashSet};

use crate::attributes::*;
use crate::index::*;
use crate::query::*;
use crate::error::*;

pub struct SearchEngine {
    indices: HashMap<String, Box<dyn SearchIndex<usize>>>,
}

impl SearchEngine {
    pub fn new(schema: &AttributeSchema) -> Self {
        let mut indices = HashMap::with_capacity(
            schema.count()
        );

        for (name, t) in schema.iter() {
            match t {
                AttributeKind::ExactMatch => {
                    indices.insert(
                        name.clone(),
                        Box::new(SearchIndexExact::<usize>::new()) as _,
                    );
                },
                AttributeKind::PrefixMatch => {
                    indices.insert(
                        name.clone(),
                        Box::new(SearchIndexPrefix::<usize>::new()) as _,
                    );
                },
                AttributeKind::RangeMatch => {
                    indices.insert(
                        name.clone(),
                        Box::new(SearchIndexRange::<usize>::new()) as _,
                    );
                },
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
        let index = self.indices.get(attribute).ok_or(SearchEngineError::UnknownArgument)?;
        Ok(index.search(attribute_value))
    }

    pub fn search(&self, query: &Query) -> Result<HashSet<usize>> {
        let mut result_set = HashSet::<usize>::new();
        for (i, pred) in query.include_predicates.iter().enumerate() {
            let attribute_set = self.search_attribute(&pred.attribute(), &pred.value())?;
            if i == 0 {
                result_set = attribute_set;
            } else {
                result_set = result_set.intersection(&attribute_set).copied().collect();
            }
            if result_set.len() == 0 {
                return Ok(result_set);
            }
        }

        for pred in query.exclude_predicates.iter() {
            let attribute_set = self.search_attribute(&pred.attribute(), &pred.value())?;
            result_set = result_set.difference(&attribute_set).copied().collect();
        }

        Ok(result_set)
    }
}
