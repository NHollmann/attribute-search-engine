use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};
use trie_rs::map::{Trie, TrieBuilder};

pub trait SearchIndex<P: Eq + Hash + Clone> {
    fn insert(&mut self, primary_id: P, attribute_value: String);
    fn search(&self, attribute_value: &str) -> HashSet<P>;
}

pub struct SearchIndexExact<P: Eq + Hash + Clone> {
    index: HashMap<String, HashSet<P>>,
}

impl<P: Eq + Hash + Clone> SearchIndexExact<P> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
}

impl<P: Eq + Hash + Clone> SearchIndex<P> for SearchIndexExact<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    fn search(&self, attribute_value: &str) -> HashSet<P> {
        self.index
            .get(attribute_value)
            .cloned()
            .unwrap_or(HashSet::<P>::new())
    }
}

pub struct SearchIndexPrefix<P: Eq + Hash + Ord> {
    // TODO we need to build the index. Maybe add a build() function to the trait?
    builder: TrieBuilder<u8, HashSet<P>>,
    index: Option<Trie<u8, HashSet<P>>>,
}

impl<P: Eq + Hash + Ord + Clone> SearchIndexPrefix<P> {
    pub fn new() -> Self {
        Self {
            builder: TrieBuilder::new(),
            index: None,
        }
    }
}

impl<P: Eq + Hash + Ord + Clone> SearchIndex<P> for SearchIndexPrefix<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        let mut hs = HashSet::new();
        hs.insert(primary_id);
        self.builder.push(attribute_value, hs);
    }

    fn search(&self, attribute_value: &str) -> HashSet<P> {
        self.index
            .as_ref()
            .unwrap() // TODO Do not simply unwrap
            .exact_match(attribute_value)
            .cloned()
            .unwrap_or(HashSet::<P>::new())
    }
}

pub struct SearchIndexRange<P: Eq + Hash + Clone> {
    index: BTreeMap<String, HashSet<P>>,
}

impl<P: Eq + Hash + Clone> SearchIndexRange<P> {
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
        }
    }
}

impl<P: Eq + Hash + Clone> SearchIndex<P> for SearchIndexRange<P> {
    fn insert(&mut self, primary_id: P, attribute_value: String) {
        self.index
            .entry(attribute_value)
            .or_default()
            .insert(primary_id);
    }

    fn search(&self, attribute_value: &str) -> HashSet<P> {
        self.index
            .get(attribute_value)
            .cloned()
            .unwrap_or(HashSet::<P>::new())
    }
}
