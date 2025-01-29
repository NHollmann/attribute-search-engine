use std::clone::Clone;
use std::cmp::Ord;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

/// Prefix tree object that is specialized in storing HashSets and accessing them by strings.
pub struct HashSetPrefixTree<P> {
    /// Root of the prefix tree
    nodes: Vec<TreeNode>,

    /// Stores all values in the tree
    values: Vec<HashSet<P>>,
}

impl<P: Eq + Hash + Clone> HashSetPrefixTree<P> {
    /// Creates a new HashSetPrefixTree object
    pub fn new() -> HashSetPrefixTree<P> {
        // Initialize the root node
        let nodes = vec![TreeNode::new(None)];
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

    /// Get a HashSet from the tree by exactly matching the key.
    pub fn get(&self, key: &str) -> Option<HashSet<P>> {
        let node_id = self.find_node(key)?;
        let value_id = self.nodes[node_id].get()?;
        Some(self.values[value_id].clone())
    }

    /// Get a HashSet from the tree by finding all entries that share the same prefix.
    pub fn get_prefix(&self, prefix: &str) -> Option<HashSet<P>> {
        let mut node_ids = VecDeque::new();
        let mut result_set = HashSet::<P>::new();

        let node_id = self.find_node(prefix)?;
        node_ids.push_back(node_id);

        while let Some(node_id) = node_ids.pop_front() {
            if let Some(value_id) = self.nodes[node_id].get() {
                result_set = result_set.union(&self.values[value_id]).cloned().collect();
            }

            node_ids.extend(self.nodes[node_id].children.iter().map(|x| x.1));
        }

        Some(result_set)
    }

    /// Find a [TreeNode] in the tree by its key.
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
    /// Create a new TreeNode.
    pub fn new(value: Option<usize>) -> TreeNode {
        TreeNode {
            value,
            children: Vec::<(char, usize)>::new(),
        }
    }

    /// Find the index of a child that matches the key.
    /// If no child is found, None is returned.
    pub fn find_child(&self, key: &char) -> Option<usize> {
        self.children
            .binary_search_by(|x| x.0.cmp(key))
            .map(|idx| self.children[idx].1)
            .ok()
    }

    /// Insert a new child and sort the children for faster access.
    pub fn insert_child(&mut self, key: char, child_id: usize) {
        self.children.push((key, child_id));
        self.children.sort_by(|a, b| a.0.cmp(&b.0));
    }

    /// Set the value of this node.
    pub fn set(&mut self, value: usize) {
        self.value = Some(value);
    }

    /// Get the current value of this node.
    /// If no value was set before, None is returned.
    pub fn get(&self) -> Option<usize> {
        self.value
    }
}
