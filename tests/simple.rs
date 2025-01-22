use std::collections::HashSet;

use attribute_search_engine::{Query, SearchEngine, SearchIndexBuilder, SearchIndexExact};

#[test]
fn basic_example() {
    let mut index_name = SearchIndexExact::<_, String>::new();
    let mut index_zipcode = SearchIndexExact::<_, String>::new();
    let mut index_city = SearchIndexExact::<_, String>::new();
    let mut index_pet = SearchIndexExact::<_, String>::new();

    index_name.insert(0, "Alice".into());
    index_zipcode.insert(0, "12345".into());
    index_city.insert(0, "New York".into());
    index_name.insert(1, "Bob".into());
    index_zipcode.insert(1, "12345".into());
    index_city.insert(1, "New York".into());
    index_pet.insert(1, "Cat".into());
    index_pet.insert(1, "Dog".into());
    index_pet.insert(1, "Bees".into());
    index_name.insert(2, "Eve".into());
    index_zipcode.insert(2, "12345".into());
    index_city.insert(2, "New York".into());
    index_zipcode.insert(2, "54321".into());
    index_pet.insert(2, "Cat".into());
    index_city.insert(2, "Berlin".into());
    index_name.insert(3, "Victor".into());
    index_city.insert(3, "Prag".into());
    index_pet.insert(3, "Dog".into());
    index_name.insert(4, "Hans".into());
    index_city.insert(4, "New York".into());
    index_zipcode.insert(4, "12345".into());
    index_pet.insert(4, "Dog".into());
    index_name.insert(5, "Peter".into());
    index_city.insert(5, "New York".into());
    index_zipcode.insert(5, "12345".into());
    index_pet.insert(5, "Dog".into());
    index_pet.insert(5, "Cat".into());

    let mut engine = SearchEngine::new();
    engine.add_index("name", index_name);
    engine.add_index("zipcode", index_zipcode);
    engine.add_index("city", index_city);
    engine.add_index("pet", index_pet);

    let q = Query::Exact("zipcode".into(), "12345".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 4, 5]));

    let q = Query::Exclude(
        Query::And(vec![
            Query::Exact("zipcode".into(), "12345".into()),
            Query::Exact("pet".into(), "Dog".into()),
        ])
        .into(),
        vec![Query::Exact("name".into(), "Hans".into())],
    );
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![1, 5]));

    let q = Query::Exclude(
        Query::Or(vec![
            Query::Exact("zipcode".into(), "12345".into()),
            Query::Exact("pet".into(), "Dog".into()),
        ])
        .into(),
        vec![Query::Exact("name".into(), "Hans".into())],
    );
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 3, 5]));

    let q = engine
        .query_from_str("+zipcode:12345 +pet:Dog -name:Hans")
        .expect("valid query");
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![1, 5]));
}
