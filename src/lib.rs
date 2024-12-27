use std::{collections::{HashMap, HashSet}, hash::Hash};

// AttributeType sets the type of an attribute in an AttributeSchema
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeType {
    /// ExactMatch attributes must match exactly to be considered
    ExactMatch,

    /// PrefixMatch attributes only need to match on the beginning
    PrefixMatch,

    /// RangeMatch attributes can be sorted and searched by range
    RangeMatch,
}

/// AttributeSchema is a collection of multiple attributes consisting
/// of a name and type. They are used to construct new SearchEngines.
///
/// # Example
///
/// ```
/// let mut schema = attribute_search_engine::AttributeSchema::new();
/// schema.register_attribute("zipcode", attribute_search_engine::AttributeType::ExactMatch);
/// schema.register_attribute("age", attribute_search_engine::AttributeType::RangeMatch);
/// ```
#[derive(Debug, Clone)]
pub struct AttributeSchema {
    attributes: Vec<(String, AttributeType)>,
}

impl AttributeSchema {
    /// Create a new AttributeSchema
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }

    /// Register a new attribute on a schema
    pub fn register_attribute(&mut self, name: &str, attr_type: AttributeType) {
        self.attributes.push((String::from(name), attr_type));
    }
}

#[derive(Debug)]
struct SearchIndexExact<P: Eq + Hash, T: Eq + Hash> {
    index: HashMap<T, HashSet<P>>,
}

impl<P: Eq + Hash, T: Eq + Hash> SearchIndexExact<P, T> {
    fn insert(&mut self, primary_id: P, attribute_value: T) {
        self.index.entry(attribute_value).or_default().insert(primary_id);
    }

    fn search(&self, attribute_value: T) -> Option<&HashSet<P>> {
        self.index.get(&attribute_value)
    }
}

#[derive(Debug)]
pub struct SearchEngine {
    indices: HashMap<String, SearchIndexExact<usize, String>>,
}

impl SearchEngine {
    pub fn new(schema: &AttributeSchema) -> Self {
        let mut indices = HashMap::with_capacity(schema.attributes.iter().filter(|x| x.1 == AttributeType::ExactMatch).count());

        for (name, t) in &schema.attributes {
            if *t != AttributeType::ExactMatch {
                panic!("only ExactMatch is supported");
            }
            indices.insert(name.clone(), SearchIndexExact { index: HashMap::new() });
        }

        Self {
            indices,
        }
    }

    pub fn insert(&mut self, primary_id: usize, attribute: &str, attribute_value: &str) {
        match self.indices.get_mut(attribute) {
            Some(index) => index.insert(primary_id, attribute_value.to_string()),
            _ => {}
        }
    }

    pub fn search_attribute(&self, attribute: &str, attribute_value: String) -> Option<&HashSet<usize>> {
        let index = self.indices.get(attribute)?;
        index.search(attribute_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut schema = AttributeSchema::new();
        schema.register_attribute("name", AttributeType::ExactMatch);
        schema.register_attribute("zipcode", AttributeType::ExactMatch);
        let mut engine = SearchEngine::new(&schema);
        engine.insert(0, "name", "Alice");
        engine.insert(0, "zipcode", "12345");
        engine.insert(1, "name", "Bob");
        engine.insert(1, "zipcode", "12345");
        engine.insert(2, "name", "Eve");
        engine.insert(2, "zipcode", "12345");
        engine.insert(2, "zipcode", "54321");

        let result = engine.search_attribute("zipcode", "12345".to_owned()).expect("search attribute result to not be empty");
        assert_eq!(result.len(), 3);
    }
}
