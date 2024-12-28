use std::collections::{HashMap, HashSet};

use crate::attributes::*;
use crate::index::*;
use crate::query::*;

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
                AttributeType::ExactMatch => {
                    indices.insert(
                        name.clone(),
                        Box::new(SearchIndexExact::<usize>::new()) as _,
                    );
                },
                AttributeType::PrefixMatch => todo!(),
                AttributeType::RangeMatch => {
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
    ) -> Option<&HashSet<usize>> {
        let index = self.indices.get(attribute)?;
        index.search(attribute_value)
    }

    pub fn search(&self, query: &Query) {
        /*for pred in query.include_predicates.iter() {
        }
        for pred in query.exclude_predicates.iter() {
        }*/
    }
}
