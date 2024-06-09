use std::cmp::Ordering;
use std::fmt::Debug;

// We set the minimum degree of the BTree
const NODE_T: usize = 2;
// This is the maximum number of keys a node can have
const MAX_NODE_SIZE: usize = 2 * NODE_T - 1;

// A structure representing a node in the BTree
#[derive(Debug)]
struct BTreeNode<T> {
    keys: Vec<T>,                             // A list to store keys in the node
    children: Vec<Option<Box<BTreeNode<T>>>>, // A list to store child nodes
    leaf: bool, // A flag to check if the node is a leaf (has no children)
}

// Implementation of the BTreeNode structure
impl<T: Ord + Clone> BTreeNode<T> {
    // Function to create a new node
    fn new(leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),     // Initialize an empty list for keys
            children: Vec::new(), // Initialize an empty list for children
            leaf,                 // Set the leaf flag
        }
    }

    // Function to split a child node when it becomes full
    fn split_child(&mut self, i: usize) {
        // Get the child node that needs to be split
        let mut y = self.children[i].take().unwrap();
        // Create a new node to hold half of the keys of the full child
        let mut z = BTreeNode::new(y.leaf);

        // Move the second half of the keys to the new node
        z.keys = y.keys.split_off(NODE_T);

        // If the child node is not a leaf, move half of its children to the new node
        if !y.leaf {
            z.children = y.children.split_off(NODE_T);
        }

        // Insert the new node as a child of the current node
        self.children.insert(i + 1, Some(Box::new(z)));
        // Move the middle key of the full child to the current node
        self.keys.insert(i, y.keys.pop().unwrap());

        // Reassign the modified child back to its position
        self.children[i] = Some(y);
    }

    // Function to insert a key into a node that is not full
    fn insert_non_full(&mut self, key: T) {
        // Start from the last key
        let mut i = self.keys.len();

        // If the node is a leaf, insert the key in the correct position
        if self.leaf {
            // Add the key at the end
            self.keys.push(key.clone());
            // Move the key to the correct position by comparing and swapping
            while i > 0 && self.keys[i - 1] > key {
                self.keys.swap(i, i - 1);
                i -= 1;
            }
        } else {
            // Find the child that will have the new key
            while i > 0 && self.keys[i - 1] > key {
                i -= 1;
            }
            // If the child is full, split it
            if self.children[i].as_ref().unwrap().keys.len() == MAX_NODE_SIZE {
                self.split_child(i);
                // After splitting, the key might go to the right child
                if self.keys[i] < key {
                    i += 1;
                }
            }
            // Insert the key into the appropriate child
            self.children[i].as_mut().unwrap().insert_non_full(key);
        }
    }
}

// A structure representing the BTree
#[derive(Debug)]
struct BTree<T> {
    root: Option<Box<BTreeNode<T>>>, // The root node of the BTree
}

// Implementation of the BTree structure
impl<T: Ord + Clone> BTree<T> {
    // Function to create a new BTree
    fn new() -> Self {
        BTree { root: None } // Initially, the BTree is empty
    }

    // Function to insert a key into the BTree
    fn insert(&mut self, key: T) {
        if let Some(root) = &mut self.root {
            // If the root node is full, split it
            if root.keys.len() == 2 * NODE_T - 1 {
                // Create a new root node
                let mut s = BTreeNode::new(false);
                // Make the old root a child of the new root
                s.children.push(self.root.take());
                // Split the old root
                s.split_child(0);
                // Insert the new key into the new root
                s.insert_non_full(key);
                // Update the root
                self.root = Some(Box::new(s));
            } else {
                // If the root is not full, insert the key into the root
                root.insert_non_full(key);
            }
        } else {
            // If the tree is empty, create a new root node and insert the key
            let mut root = BTreeNode::new(true);
            root.keys.push(key);
            self.root = Some(Box::new(root));
        }
    }

    // Function to search for a key in the BTree
    fn search(&self, key: &T) -> Option<&T> {
        self.search_in_node(&self.root, key) // Start searching from the root
    }

    // Helper function to search for a key in a node
    fn search_in_node<'a>(&'a self, node: &'a Option<Box<BTreeNode<T>>>, key: &T) -> Option<&'a T> {
        if let Some(node) = node {
            let mut i = 0;
            // Find the first key greater than or equal to the target key
            while i < node.keys.len() && *key > node.keys[i] {
                i += 1;
            }
            // If the key is found, return it
            if i < node.keys.len() && *key == node.keys[i] {
                return Some(&node.keys[i]);
            } else if node.leaf {
                // If the node is a leaf, the key is not in the tree
                return None;
            } else {
                // Otherwise, search in the appropriate child
                return self.search_in_node(&node.children[i], key);
            }
        }
        None
    }
}

#[test]
fn test() {
    let mut btree = BTree::new();
    btree.insert(10);
    btree.insert(20);
    btree.insert(5);
    btree.insert(6);
    btree.insert(12);
    btree.insert(30);
    btree.insert(7);
    btree.insert(17);

    println!("{:?}", btree);

    if let Some(value) = btree.search(&6) {
        println!("Found: {}", value);
    } else {
        println!("Not Found");
    }
}
