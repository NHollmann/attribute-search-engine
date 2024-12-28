mod attributes;
mod engine;
mod error;
mod index;
mod query;

pub use attributes::*;
pub use engine::*;
pub use error::*;
pub use query::Query;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn basic_example() {
        let mut schema = AttributeSchema::new();
        schema.register_attribute("name", AttributeKind::ExactMatch);
        schema.register_attribute("zipcode", AttributeKind::ExactMatch);
        schema.register_attribute("city", AttributeKind::ExactMatch);
        schema.register_attribute("pet", AttributeKind::ExactMatch);
        let mut engine = SearchEngine::new(&schema);
        engine.insert(0, "name", "Alice");
        engine.insert(0, "zipcode", "12345");
        engine.insert(0, "city", "New York");
        engine.insert(1, "name", "Bob");
        engine.insert(1, "zipcode", "12345");
        engine.insert(1, "city", "New York");
        engine.insert(1, "pet", "Cat");
        engine.insert(1, "pet", "Dog");
        engine.insert(1, "pet", "Bees");
        engine.insert(2, "name", "Eve");
        engine.insert(2, "zipcode", "12345");
        engine.insert(2, "city", "New York");
        engine.insert(2, "zipcode", "54321");
        engine.insert(2, "pet", "Cat");
        engine.insert(2, "city", "Berlin");
        engine.insert(3, "name", "Victor");
        engine.insert(3, "city", "Prag");
        engine.insert(3, "pet", "Dog");
        engine.insert(4, "name", "Hans");
        engine.insert(4, "city", "New York");
        engine.insert(4, "zipcode", "12345");
        engine.insert(4, "pet", "Dog");
        engine.insert(5, "name", "Peter");
        engine.insert(5, "city", "New York");
        engine.insert(5, "zipcode", "12345");
        engine.insert(5, "pet", "Dog");
        engine.insert(5, "pet", "Cat");

        let result = engine
            .search_attribute("zipcode", "12345")
            .expect("search attribute result to not be empty");
        assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 4, 5]));

        let q = Query::new().
            include_equal("zipcode", "12345").
            include_equal("pet", "Dog").
            exclude_equal("name", "Hans");
        let result = engine.search(&q).expect("no errors during search");
        assert_eq!(result, HashSet::from_iter(vec![1, 5]));
    }
}
