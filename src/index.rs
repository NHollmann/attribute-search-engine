use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};

pub trait SearchIndex<P: Eq + Hash> {
    fn insert(&mut self, primary_id: P, attribute_value: String);
    fn search(&self, attribute_value: &str) -> Option<&HashSet<P>>;
}

pub struct SearchIndexExact<P: Eq + Hash> {
    index: HashMap<String, HashSet<P>>,
}

impl<P: Eq + Hash> SearchIndexExact<P> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
}

impl<P: Eq + Hash> SearchIndex<P> for SearchIndexExact<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    fn search(&self, attribute_value: &str) -> Option<&HashSet<P>> {
        self.index.get(attribute_value)
    }
}

pub struct SearchIndexRange<P: Eq + Hash> {
    index: BTreeMap<String, HashSet<P>>,
}

impl<P: Eq + Hash> SearchIndexRange<P> {
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
        }
    }
}

impl<P: Eq + Hash> SearchIndex<P> for SearchIndexRange<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    fn search(&self, attribute_value: &str) -> Option<&HashSet<P>> {
        self.index.get(attribute_value)
    }
}
