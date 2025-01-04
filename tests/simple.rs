use std::collections::HashSet;

use attribute_search_engine::{AttributeKind, AttributeSchema, Query, QueryValue, SearchEngine};

#[test]
fn basic_example() {
    let mut schema = AttributeSchema::new();
    schema.register_attribute("name", AttributeKind::ExactMatch);
    schema.register_attribute("zipcode", AttributeKind::ExactMatch);
    schema.register_attribute("city", AttributeKind::ExactMatch);
    schema.register_attribute("pet", AttributeKind::ExactMatch);
    let mut engine = SearchEngine::new(&schema);
    engine.insert(0, "name", "Alice").unwrap();
    engine.insert(0, "zipcode", "12345").unwrap();
    engine.insert(0, "city", "New York").unwrap();
    engine.insert(1, "name", "Bob").unwrap();
    engine.insert(1, "zipcode", "12345").unwrap();
    engine.insert(1, "city", "New York").unwrap();
    engine.insert(1, "pet", "Cat").unwrap();
    engine.insert(1, "pet", "Dog").unwrap();
    engine.insert(1, "pet", "Bees").unwrap();
    engine.insert(2, "name", "Eve").unwrap();
    engine.insert(2, "zipcode", "12345").unwrap();
    engine.insert(2, "city", "New York").unwrap();
    engine.insert(2, "zipcode", "54321").unwrap();
    engine.insert(2, "pet", "Cat").unwrap();
    engine.insert(2, "city", "Berlin").unwrap();
    engine.insert(3, "name", "Victor").unwrap();
    engine.insert(3, "city", "Prag").unwrap();
    engine.insert(3, "pet", "Dog").unwrap();
    engine.insert(4, "name", "Hans").unwrap();
    engine.insert(4, "city", "New York").unwrap();
    engine.insert(4, "zipcode", "12345").unwrap();
    engine.insert(4, "pet", "Dog").unwrap();
    engine.insert(5, "name", "Peter").unwrap();
    engine.insert(5, "city", "New York").unwrap();
    engine.insert(5, "zipcode", "12345").unwrap();
    engine.insert(5, "pet", "Dog").unwrap();
    engine.insert(5, "pet", "Cat").unwrap();

    let q = Query::Exact("zipcode".into(), QueryValue::Str("12345".into()));
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 4, 5]));

    let q = Query::Exclude(
        Query::And(vec![
            Query::Exact("zipcode".into(), QueryValue::Str("12345".into())),
            Query::Exact("pet".into(), QueryValue::Str("Dog".into())),
        ])
        .into(),
        vec![Query::Exact("name".into(), QueryValue::Str("Hans".into()))],
    );
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![1, 5]));

    let q = Query::Exclude(
        Query::Or(vec![
            Query::Exact("zipcode".into(), QueryValue::Str("12345".into())),
            Query::Exact("pet".into(), QueryValue::Str("Dog".into())),
        ])
        .into(),
        vec![Query::Exact("name".into(), QueryValue::Str("Hans".into()))],
    );
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 3, 5]));

    let q = engine
        .query_from_str("+zipcode:12345 +pet:Dog -name:Hans")
        .expect("valid query");
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![1, 5]));
}
