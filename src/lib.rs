use std::collections::HashMap;

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
#[derive(Clone)]
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

struct SearchIndexExact<T> {
    index: HashMap<String, T>,
}

pub struct SearchEngine {
    indices: HashMap<String, SearchIndexExact<String>>,
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
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
