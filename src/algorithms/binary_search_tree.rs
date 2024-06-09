// write binary search tree
#[derive(PartialEq, Debug, Clone)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone + Default> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        if value <= self.value {
            match &mut self.left {
                Some(node) => node.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match &mut self.right {
                Some(node) => node.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    pub fn search(&self, value: T) -> Option<&Node<T>> {
        if value == self.value {
            return Some(self);
        }

        if value < self.value {
            match &self.left {
                Some(node) => node.search(value),
                None => None,
            }
        } else {
            match &self.right {
                Some(node) => node.search(value),
                None => None,
            }
        }
    }
    pub fn delete(&mut self, value: T) -> Option<Box<Node<T>>> {
        match self.value.cmp(&value) {
            std::cmp::Ordering::Less => {
                if let Some(ref mut right) = self.right {
                    self.right = right.delete(value);
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(ref mut left) = self.left {
                    self.left = left.delete(value);
                }
            }
            std::cmp::Ordering::Equal => {
                if self.left.is_none() {
                    return self.right.take();
                } else if self.right.is_none() {
                    return self.left.take();
                } else {
                    // Node with two children: Get the in-order successor (smallest in the right subtree)
                    let mut successor = self.right.as_mut().unwrap();
                    while let Some(ref mut left) = successor.left {
                        successor = left;
                    }
                    self.value = successor.value.clone();
                    self.right = self.right.take().unwrap().delete(self.value.clone());
                }
            }
        }
        // Return the current node if it wasn't deleted
        Some(Box::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_tree() {
        let mut root = Node::new(5);
        root.insert(3);
        root.insert(7);
        root.insert(2);
        root.insert(4);
        root.insert(6);
        root.insert(8);

        assert_eq!(root.search(5).unwrap().value, 5);
        assert_eq!(root.search(3).unwrap().value, 3);
    }

    #[test]
    fn test_binary_search_tree_delete() {
        let mut root = Node::new(50);
        root.insert(30);
        root.insert(20);
        root.insert(40);
        root.insert(70);
        root.insert(60);
        root.insert(80);

        // Test deleting a leaf node
        root.delete(20);

        // Test deleting a node with one child
        root.delete(30);

        // Test deleting a node with two children
        root.delete(50);

        assert_eq!(root.search(50), None);
        assert_eq!(root.search(30), None);
        assert_eq!(root.search(20), None);
    }
}
