use std::clone::Clone;
use std::cmp::Ord;
use std::collections::HashSet;
use std::hash::Hash;

/// Prefix tree object that is specialized in storing HashSets and accessing them by strings.
pub struct HashSetPrefixTree<P> {
    /// Root of the prefix tree
    nodes: Vec<TreeNode>,

    /// Stores all values in the tree
    values: Vec<HashSet<P>>,
}

impl<P: Eq + Hash + Clone> HashSetPrefixTree<P> {
    /// Creates a new `HashSetPrefixTree` object
    pub fn new() -> HashSetPrefixTree<P> {
        let mut nodes = Vec::<TreeNode>::new();
        // Initialize the root node
        nodes.push(TreeNode::new(None));

        HashSetPrefixTree {
            nodes,
            values: Vec::<HashSet<P>>::new(),
        }
    }

    /// Adds a new value to the tree.
    ///
    /// If no entry under this key exists, a new HashSet will be created.
    /// If there is already an entry, the new value will be added to the existing set.
    pub fn insert(&mut self, key: &str, value: P) {
        let mut node_id = 0usize;

        for c in key.chars() {
            if let Some(id) = self.nodes[node_id].find_child(&c) {
                node_id = id;
            } else {
                let new_node_id = self.create_new_node();
                self.nodes[node_id].insert_child(c, new_node_id);
                node_id = new_node_id;
            }
        }

        let value_id = match self.nodes[node_id].get() {
            Some(id) => {
                self.values[id].insert(value);
                id
            }
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(value);
                self.values.push(new_set);
                self.values.len() - 1
            }
        };

        self.nodes[node_id].set(value_id);
    }

    /// Get a HashSet from the tree.
    pub fn get(&self, key: &str) -> Option<HashSet<P>> {
        let node_id = self.find_node(key)?;
        let value_id = self.nodes[node_id].get()?;
        Some(self.values[value_id].clone())
    }

    /// Find a `TreeNode` in the tree by its key.
    fn find_node(&self, key: &str) -> Option<usize> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut node_id = 0usize;
        for c in key.chars() {
            node_id = self.nodes[node_id].find_child(&c)?;
        }
        Some(node_id)
    }

    /// Create a new node
    fn create_new_node(&mut self) -> usize {
        self.nodes.push(TreeNode::new(None));
        self.nodes.len() - 1
    }
}

/// A single node in the prefix tree.
struct TreeNode {
    /// Index of the value in the trees value vector.
    pub value: Option<usize>,

    /// Children of this sub-tree.
    pub children: Vec<(char, usize)>,
}

impl TreeNode {
    pub fn new(value: Option<usize>) -> TreeNode {
        TreeNode {
            value,
            children: Vec::<(char, usize)>::new(),
        }
    }

    pub fn find_child(&self, key: &char) -> Option<usize> {
        self.children
            .binary_search_by(|x| x.0.cmp(key))
            .map(|idx| self.children[idx].1)
            .ok()
    }

    pub fn insert_child(&mut self, key: char, child_id: usize) {
        self.children.push((key, child_id));
        self.children.sort_by(|a, b| a.0.cmp(&b.0));
    }

    pub fn set(&mut self, value: usize) {
        self.value = Some(value);
    }

    pub fn get(&self) -> Option<usize> {
        self.value
    }
}
