use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};
use trie_rs::map::{TrieBuilder, Trie};

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

pub struct SearchIndexPrefix<P: Eq + Hash + Ord> {
    // TODO we need to build the index. Maybe add a build() function to the trait?
    builder: TrieBuilder<u8, HashSet<P>>,
    index: Option<Trie<u8, HashSet<P>>>,
}

impl<P: Eq + Hash + Ord> SearchIndexPrefix<P> {
    pub fn new() -> Self {
        Self {
            builder: TrieBuilder::new(),
            index: None,
        }
    }
}

impl<P: Eq + Hash + Ord> SearchIndex<P> for SearchIndexPrefix<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        let mut hs = HashSet::new();
        hs.insert(primary_id);
        self.builder.push(attribute_value, hs);
    }

    fn search(&self, attribute_value: &str) -> Option<&HashSet<P>> {
        self.index.as_ref()?.exact_match(attribute_value)
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
