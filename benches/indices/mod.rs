use attribute_search_engine::*;

pub fn create_index_hashmap(input: &[String]) -> SearchIndexHashMap<usize, String> {
    let mut index = SearchIndexHashMap::<_, _>::new();

    for (i, val) in input.iter().enumerate() {
        index.insert(i, val.clone());
    }

    index
}

pub fn create_index_prefix_tree(input: &[String]) -> SearchIndexPrefixTree<usize> {
    let mut index = SearchIndexPrefixTree::<_>::new();

    for (i, val) in input.iter().enumerate() {
        index.insert(i, val.clone());
    }

    index
}

pub fn create_index_btree_range(input: &[String]) -> SearchIndexBTreeRange<usize, String> {
    let mut index = SearchIndexBTreeRange::<_, _>::new();

    for (i, val) in input.iter().enumerate() {
        index.insert(i, val.clone());
    }

    index
}
