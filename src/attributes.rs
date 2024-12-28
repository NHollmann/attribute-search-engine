
/// AttributeType sets the type of an attribute in an AttributeSchema
#[derive(Clone, Copy, PartialEq)]
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

    /// Get the count of attributes in this schema
    pub fn count(&self) -> usize {
        self.attributes.len()
    }

    /// Return an iterator over all attributes
    pub fn iter(&self) -> core::slice::Iter<(String, AttributeType)> {
        self.attributes.iter()
    }
}