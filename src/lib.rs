mod attributes;
mod index;
mod engine;
mod query;

pub use attributes::*;
pub use engine::*;

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

        let result = engine
            .search_attribute("zipcode", "12345")
            .expect("search attribute result to not be empty");
        assert_eq!(result.len(), 3);
    }
}
