use std::collections::HashSet;

use attribute_search_engine::{
    Query, SearchEngine, SearchIndexBTreeRange, SearchIndexHashMap, SearchIndexPrefixTree,
};

#[test]
fn query_btree_range_index() {
    let engine = create_person_search_engine();

    let q = Query::Exact("age".into(), "27".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1,]));

    let q = Query::InRange("age".into(), "24".into(), "34".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 3, 4]));

    let q = Query::OutRange("age".into(), "25".into(), "34".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![2, 5]));

    let q = Query::Minimum("age".into(), "27".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 4, 5]));

    let q = Query::Maximum("age".into(), "27".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 3]));
}

#[test]
fn query_hashmap_index() {
    let engine = create_person_search_engine();

    let q = Query::Exact("name".into(), "Bob".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![1]));

    let q = Query::Exact("zipcode".into(), "12345".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 4, 5]));

    let q = Query::Exact("city".into(), "Frankfurt".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![]));
}

#[test]
fn query_prefix_tree_index() {
    let engine = create_person_search_engine();

    let q = Query::Exact("permission".into(), "dashboard.show".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2, 3, 4, 5]));

    let q = Query::Exact("permission".into(), "finances".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![]));

    let q = Query::Exact("permission".into(), "personel.read".into());
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1, 2]));
}

#[test]
fn query_advanced() {
    let engine = create_person_search_engine();

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
}

#[test]
fn query_parser() {
    let engine = create_person_search_engine();

    let q = engine
        .query_from_str("+zipcode:12345 +pet:Dog -name:Hans")
        .expect("valid query");
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![1, 5]));

    let q = engine.query_from_str("+age:27").expect("valid query");
    let result = engine.search(&q).expect("no errors during search");
    assert_eq!(result, HashSet::from_iter(vec![0, 1]));
}

fn create_person_search_engine() -> SearchEngine<u8> {
    let mut index_name = SearchIndexHashMap::<_, String>::new();
    let mut index_zipcode = SearchIndexHashMap::<_, String>::new();
    let mut index_city = SearchIndexHashMap::<_, String>::new();
    let mut index_pet = SearchIndexHashMap::<_, String>::new();
    let mut index_age = SearchIndexBTreeRange::<_, u8>::new();
    let mut index_permission = SearchIndexPrefixTree::<_>::new();

    index_name.insert(0, "Alice".into());
    index_zipcode.insert(0, "12345".into());
    index_city.insert(0, "New York".into());
    index_age.insert(0, 27);
    index_permission.insert(0, "dashboard.show".into());
    index_permission.insert(0, "finances.read".into());
    index_permission.insert(0, "finances.write".into());
    index_permission.insert(0, "personel.read".into());

    index_name.insert(1, "Bob".into());
    index_zipcode.insert(1, "12345".into());
    index_city.insert(1, "New York".into());
    index_pet.insert(1, "Cat".into());
    index_pet.insert(1, "Dog".into());
    index_pet.insert(1, "Bees".into());
    index_age.insert(1, 27);
    index_permission.insert(1, "dashboard.show".into());
    index_permission.insert(1, "finances.read".into());
    index_permission.insert(1, "personel.read".into());

    index_name.insert(2, "Eve".into());
    index_zipcode.insert(2, "12345".into());
    index_city.insert(2, "New York".into());
    index_zipcode.insert(2, "54321".into());
    index_pet.insert(2, "Cat".into());
    index_city.insert(2, "Berlin".into());
    index_age.insert(2, 23);
    index_permission.insert(2, "dashboard.show".into());
    index_permission.insert(2, "personel.read".into());
    index_permission.insert(2, "personel.write".into());

    index_name.insert(3, "Victor".into());
    index_city.insert(3, "Prag".into());
    index_pet.insert(3, "Dog".into());
    index_age.insert(3, 25);
    index_permission.insert(3, "dashboard.show".into());

    index_name.insert(4, "Hans".into());
    index_city.insert(4, "New York".into());
    index_zipcode.insert(4, "12345".into());
    index_pet.insert(4, "Dog".into());
    index_age.insert(4, 34);
    index_permission.insert(4, "dashboard.show".into());
    index_permission.insert(4, "finances.read".into());

    index_name.insert(5, "Peter".into());
    index_city.insert(5, "New York".into());
    index_zipcode.insert(5, "12345".into());
    index_pet.insert(5, "Dog".into());
    index_pet.insert(5, "Cat".into());
    index_age.insert(5, 51);
    index_permission.insert(5, "dashboard.show".into());
    index_permission.insert(5, "finances.read".into());
    index_permission.insert(5, "finances.write".into());
    index_permission.insert(5, "finances.share".into());

    let mut engine = SearchEngine::new();
    engine.add_index("name", index_name);
    engine.add_index("zipcode", index_zipcode);
    engine.add_index("city", index_city);
    engine.add_index("pet", index_pet);
    engine.add_index("age", index_age);
    engine.add_index("permission", index_permission);

    engine
}
