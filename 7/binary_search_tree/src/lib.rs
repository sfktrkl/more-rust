pub mod binary_search_tree {
    use bst_node::bst_node::*;
    use std::cmp::PartialOrd;
    use std::fmt::Debug;
    use std::rc::Rc;

    #[derive(PartialEq, Debug)]
    pub struct BinarySearchTree<T> {
        root: BSTNodeOption<T>,
    }

    impl<T: PartialOrd + Debug + Copy> BinarySearchTree<T> {
        pub fn new() -> Self {
            BinarySearchTree { root: None }
        }

        fn insert_node(&self, mut bst_node: BSTNodeOption<T>, value: T) -> BSTNodeOption<T> {
            match &bst_node {
                Some(node) => {
                    let node_value = node.borrow().value;
                    // If the given value is greater than
                    // node's value then go to right subtree
                    if node_value < value {
                        let right = self.insert_node(
                            match &node.borrow().right {
                                Some(right) => Some(Rc::clone(right)),
                                _ => None,
                            },
                            value,
                        );
                        node.borrow_mut().right = right;
                        match &node.borrow().right {
                            Some(right) => right.borrow_mut().parent = Some(Rc::clone(node)),
                            _ => (),
                        }
                    }
                    // If the given value is smaller than
                    // node's value then go to left subtree
                    else {
                        let left = self.insert_node(
                            match &node.borrow().left {
                                Some(left) => Some(Rc::clone(left)),
                                _ => None,
                            },
                            value,
                        );
                        node.borrow_mut().left = left;
                        match &node.borrow().left {
                            Some(left) => left.borrow_mut().parent = Some(Rc::clone(node)),
                            _ => (),
                        }
                    }
                }
                // If BST doesn't exist create a new node as root or it's reached
                // when there's no any child node so we can insert a new node.
                _ => bst_node = BSTNode::new(value, None, None, None),
            }
            bst_node
        }

        pub fn insert(&mut self, value: T) {
            self.root = self.insert_node(
                match &self.root {
                    Some(root) => Some(Rc::clone(root)),
                    _ => None,
                },
                value,
            );
        }

        fn print_inorder_node(&self, bst_node: BSTNodeOption<T>) {
            match &bst_node {
                Some(node) => {
                    // Get the smallest value first
                    // which is in the left subtree
                    self.print_inorder_node(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    });
                    print!("{:?} ", node.borrow().value);
                    // Continue to the greatest value
                    // which is in the right subtree
                    self.print_inorder_node(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    });
                }
                _ => (),
            }
        }

        pub fn print_inorder(&self) {
            self.print_inorder_node(match &self.root {
                Some(root) => Some(Rc::clone(root)),
                _ => None,
            });
            println!("");
        }

        fn search_node(&self, bst_node: BSTNodeOption<T>, value: T) -> BSTNodeOption<T> {
            match &bst_node {
                Some(node) => {
                    let node_value = node.borrow().value;
                    // The given value is found
                    if node_value == value {
                        Some(Rc::clone(node))
                    }
                    // The given is greater than current node's value
                    else if node_value < value {
                        self.search_node(
                            match &node.borrow().right {
                                Some(right) => Some(Rc::clone(right)),
                                _ => None,
                            },
                            value,
                        )
                    }
                    // The given is smaller than current node's value
                    else {
                        self.search_node(
                            match &node.borrow().left {
                                Some(left) => Some(Rc::clone(left)),
                                _ => None,
                            },
                            value,
                        )
                    }
                }
                _ => None,
            }
        }

        pub fn search(&self, value: T) -> bool {
            match &self.search_node(
                match &self.root {
                    Some(root) => Some(Rc::clone(root)),
                    _ => None,
                },
                value,
            ) {
                Some(_) => true,
                _ => false,
            }
        }

        fn find_min_value(&self, bst_node: BSTNodeOption<T>) -> Option<T> {
            match &bst_node {
                Some(node) => match &node.borrow().left {
                    Some(left) => self.find_min_value(Some(Rc::clone(left))),
                    _ => Some(node.borrow().value),
                },
                _ => None,
            }
        }

        pub fn find_min(&self) -> Option<T> {
            self.find_min_value(match &self.root {
                Some(root) => Some(Rc::clone(root)),
                _ => None,
            })
        }

        fn find_max_value(&self, bst_node: BSTNodeOption<T>) -> Option<T> {
            match &bst_node {
                Some(node) => match &node.borrow().right {
                    Some(right) => self.find_max_value(Some(Rc::clone(right))),
                    _ => Some(node.borrow().value),
                },
                _ => None,
            }
        }

        pub fn find_max(&self) -> Option<T> {
            self.find_max_value(match &self.root {
                Some(root) => Some(Rc::clone(root)),
                _ => None,
            })
        }

        fn successor_value(&self, bst_node: BSTNodeOption<T>) -> Option<T> {
            match &bst_node {
                // The successor is the minimum key value of right subtree
                Some(node) => match &node.borrow().right {
                    Some(right) => self.find_min_value(Some(Rc::clone(right))),
                    _ => {
                        let mut parent_node = match &node.borrow().parent {
                            Some(parent) => Some(Rc::clone(parent)),
                            _ => None,
                        };
                        let mut current_node = Some(Rc::clone(node));
                        // If current node is not root and current node is its
                        // right children continue moving up
                        loop {
                            match &parent_node {
                                Some(parent) => {
                                    let right_value = match &parent.borrow().right {
                                        Some(node) => Some(node.borrow().value),
                                        _ => None,
                                    };

                                    let current_value = match &current_node {
                                        Some(node) => Some(node.borrow().value),
                                        _ => None,
                                    };

                                    if current_value != right_value {
                                        break;
                                    }
                                }
                                _ => break,
                            }
                            current_node = parent_node;
                            parent_node = match &current_node {
                                Some(node) => match &node.borrow().parent {
                                    Some(parent) => Some(Rc::clone(parent)),
                                    _ => None,
                                },
                                _ => None,
                            };
                        }
                        // If parent node is not None then the key of parent node
                        // is the successor of node
                        match &parent_node {
                            Some(node) => Some(node.borrow().value),
                            _ => None,
                        }
                    }
                },
                _ => None,
            }
        }

        pub fn successor(&self, value: T) -> Option<T> {
            match &self.search_node(
                match &self.root {
                    Some(root) => Some(Rc::clone(root)),
                    _ => None,
                },
                value,
            ) {
                Some(node) => self.successor_value(Some(Rc::clone(node))),
                _ => None,
            }
        }

        fn predecessor_value(&self, bst_node: BSTNodeOption<T>) -> Option<T> {
            match &bst_node {
                // The predecessor is the maximum key value of left subtree
                Some(node) => match &node.borrow().left {
                    Some(left) => self.find_max_value(Some(Rc::clone(left))),
                    _ => {
                        let mut parent_node = match &node.borrow().parent {
                            Some(parent) => Some(Rc::clone(parent)),
                            _ => None,
                        };
                        let mut current_node = Some(Rc::clone(node));
                        // If current node is not root and current node is its
                        // left children continue moving up
                        loop {
                            match &parent_node {
                                Some(parent) => {
                                    let left_value = match &parent.borrow().left {
                                        Some(node) => Some(node.borrow().value),
                                        _ => None,
                                    };

                                    let current_value = match &current_node {
                                        Some(node) => Some(node.borrow().value),
                                        _ => None,
                                    };

                                    if current_value != left_value {
                                        break;
                                    }
                                }
                                _ => break,
                            }
                            current_node = parent_node;
                            parent_node = match &current_node {
                                Some(node) => match &node.borrow().parent {
                                    Some(parent) => Some(Rc::clone(parent)),
                                    _ => None,
                                },
                                _ => None,
                            };
                        }
                        // If parent node is not None then the key of parent node
                        // is the predecessor of node
                        match &parent_node {
                            Some(node) => Some(node.borrow().value),
                            _ => None,
                        }
                    }
                },
                _ => None,
            }
        }

        pub fn predecessor(&self, value: T) -> Option<T> {
            match &self.search_node(
                match &self.root {
                    Some(root) => Some(Rc::clone(root)),
                    _ => None,
                },
                value,
            ) {
                Some(node) => self.predecessor_value(Some(Rc::clone(node))),
                _ => None,
            }
        }

        fn remove_node(&self, mut bst_node: BSTNodeOption<T>, value: T) -> BSTNodeOption<T> {
            match &bst_node {
                Some(node) => {
                    let mut node_value = node.borrow().value;
                    // Target node is found
                    if node_value == value {
                        let mut have_two_children: bool = false;
                        let mut right_node: BSTNodeOption<T> = None;
                        let new_node = match &node.borrow().left {
                            Some(left) => match &node.borrow().right {
                                // The node have two children (left and right)
                                Some(right) => match self.successor(value) {
                                    Some(value) => {
                                        have_two_children = true;
                                        right_node =
                                            self.remove_node(Some(Rc::clone(right)), value);
                                        node_value = value;
                                        Some(Rc::clone(node))
                                    }
                                    _ => None,
                                },
                                // The node have only one child at left
                                _ => {
                                    left.borrow_mut().parent = match &node.borrow().parent {
                                        Some(parent) => Some(Rc::clone(parent)),
                                        _ => None,
                                    };
                                    Some(Rc::clone(left))
                                }
                            },
                            // The node have only one child at right
                            _ => match &node.borrow().right {
                                Some(right) => {
                                    right.borrow_mut().parent = match &node.borrow().parent {
                                        Some(parent) => Some(Rc::clone(parent)),
                                        _ => None,
                                    };
                                    Some(Rc::clone(right))
                                }
                                // If the node is a leaf node
                                // The node can be safely removed
                                _ => None,
                            },
                        };
                        if have_two_children {
                            node.borrow_mut().value = node_value;
                            node.borrow_mut().right = right_node;
                        }
                        bst_node = match &new_node {
                            Some(node) => Some(Rc::clone(node)),
                            _ => None,
                        }
                    }
                    // Target node's value is smaller than
                    // the given value then search to right
                    else if node_value < value {
                        let right = self.remove_node(
                            match &node.borrow().right {
                                Some(right) => Some(Rc::clone(right)),
                                _ => None,
                            },
                            value,
                        );
                        node.borrow_mut().right = right;
                    }
                    // Target node's key is greater than
                    // the given key then search to left
                    else {
                        let left = self.remove_node(
                            match &node.borrow().left {
                                Some(left) => Some(Rc::clone(left)),
                                _ => None,
                            },
                            value,
                        );
                        node.borrow_mut().left = left;
                    }

                    bst_node
                }
                _ => None,
            }
        }

        pub fn remove(&mut self, value: T) {
            self.root = self.remove_node(
                match &self.root {
                    Some(root) => Some(Rc::clone(root)),
                    _ => None,
                },
                value,
            );
        }
    }

    impl<T> Drop for BinarySearchTree<T> {
        fn drop(&mut self) {
            match &self.root.take() {
                Some(node) => {
                    let _ = node.borrow_mut().parent.take();
                    let mut right = node.borrow_mut().right.take();
                    while let Some(node) = right.take() {
                        let _ = node.borrow_mut().parent.take();
                        right = node.borrow_mut().right.take();
                        let mut left = node.borrow_mut().left.take();
                        while let Some(node) = left.take() {
                            let _ = node.borrow_mut().parent.take();
                            left = node.borrow_mut().left.take();
                        }
                    }
                    let mut left = node.borrow_mut().left.take();
                    while let Some(node) = left.take() {
                        let _ = node.borrow_mut().parent.take();
                        left = node.borrow_mut().left.take();
                        let mut right = node.borrow_mut().right.take();
                        while let Some(node) = right.take() {
                            let _ = node.borrow_mut().parent.take();
                            right = node.borrow_mut().right.take();
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
