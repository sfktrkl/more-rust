pub mod avl_tree {
    use avl_node::avl_node::*;
    use std::cmp;
    use std::cmp::PartialOrd;
    use std::fmt::Debug;
    use std::rc::Rc;

    #[derive(PartialEq, Debug)]
    pub struct AVLTree<T> {
        root: AVLNodeOption<T>,
    }

    impl<T: PartialOrd + Debug + Copy> AVLTree<T> {
        pub fn new() -> Self {
            AVLTree { root: None }
        }

        fn get_node_height(&self, avl_node: AVLNodeOption<T>) -> Option<i32> {
            match &avl_node {
                Some(node) => Some(node.borrow().height),
                _ => None,
            }
        }

        fn rotate_left(&self, avl_node: AVLNodeOption<T>) -> AVLNodeOption<T> {
            // The node must have a right child

            // Create a new node as a result that will be a balanced node
            let balanced_node = match &avl_node {
                Some(node) => match &node.borrow().right {
                    Some(right) => Some(Rc::clone(right)),
                    _ => None,
                },
                _ => None,
            };

            // The balanced node will replace the current node
            match &balanced_node {
                Some(node) => {
                    node.borrow_mut().parent = match &avl_node {
                        Some(node) => match &node.borrow().parent {
                            Some(parent) => Some(Rc::clone(parent)),
                            _ => None,
                        },
                        _ => None,
                    }
                }
                _ => (),
            }

            // The current node will be child of the balanced node
            match &avl_node {
                Some(node) => {
                    node.borrow_mut().parent = match &balanced_node {
                        Some(node) => Some(Rc::clone(node)),
                        _ => None,
                    }
                }
                _ => (),
            }

            // The right child of current node will be balanced node's left child
            match &avl_node {
                Some(node) => {
                    node.borrow_mut().right = match &balanced_node {
                        Some(node) => match &node.borrow().left {
                            Some(left) => Some(Rc::clone(left)),
                            _ => None,
                        },
                        _ => None,
                    }
                }
                _ => (),
            }

            // If balanced node has left child point the parent to the current node
            match &balanced_node {
                Some(node) => match &node.borrow().left {
                    Some(left) => {
                        left.borrow_mut().parent = match &avl_node {
                            Some(node) => Some(Rc::clone(node)),
                            _ => None,
                        }
                    }
                    _ => (),
                },
                _ => (),
            }

            // The left child of balanced node will be the current node
            match &balanced_node {
                Some(node) => {
                    node.borrow_mut().left = match &avl_node {
                        Some(node) => Some(Rc::clone(node)),
                        _ => None,
                    }
                }
                _ => (),
            }

            // Refresh the node's height
            match &avl_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };

                    node.borrow_mut().height = cmp::max(left_height, right_height) + 1;
                }
                _ => (),
            }

            // Refresh the balanced node's height
            match &balanced_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };

                    node.borrow_mut().height = cmp::max(left_height, right_height) + 1;
                }
                _ => (),
            }

            balanced_node
        }

        fn rotate_right(&self, avl_node: AVLNodeOption<T>) -> AVLNodeOption<T> {
            // The node must have a left child

            // Create a new node as a result that will be a balanced node
            let balanced_node = match &avl_node {
                Some(node) => match &node.borrow().left {
                    Some(left) => Some(Rc::clone(left)),
                    _ => None,
                },
                _ => None,
            };

            // The balanced node will replace the current node
            match &balanced_node {
                Some(node) => {
                    node.borrow_mut().parent = match &avl_node {
                        Some(node) => match &node.borrow().parent {
                            Some(parent) => Some(Rc::clone(parent)),
                            _ => None,
                        },
                        _ => None,
                    }
                }
                _ => (),
            }

            // The current node will be child of the balanced node
            match &avl_node {
                Some(node) => {
                    node.borrow_mut().parent = match &balanced_node {
                        Some(node) => Some(Rc::clone(node)),
                        _ => None,
                    }
                }
                _ => (),
            }

            // The left child of current node will be balanced node's right child
            match &avl_node {
                Some(node) => {
                    node.borrow_mut().left = match &balanced_node {
                        Some(node) => match &node.borrow().right {
                            Some(right) => Some(Rc::clone(right)),
                            _ => None,
                        },
                        _ => None,
                    }
                }
                _ => (),
            }

            // If balanced node has right child point the parent to the current node
            match &balanced_node {
                Some(node) => match &node.borrow().right {
                    Some(left) => {
                        left.borrow_mut().parent = match &avl_node {
                            Some(node) => Some(Rc::clone(node)),
                            _ => None,
                        }
                    }
                    _ => (),
                },
                _ => (),
            }

            // The right child of balanced node will be the current node
            match &balanced_node {
                Some(node) => {
                    node.borrow_mut().right = match &avl_node {
                        Some(node) => Some(Rc::clone(node)),
                        _ => None,
                    }
                }
                _ => (),
            }

            // Refresh the node's height
            match &avl_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };

                    node.borrow_mut().height = cmp::max(left_height, right_height) + 1;
                }
                _ => (),
            }

            // Refresh the balanced node's height
            match &balanced_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };

                    node.borrow_mut().height = cmp::max(left_height, right_height) + 1;
                }
                _ => (),
            }

            balanced_node
        }

        fn insert_node(&self, mut avl_node: AVLNodeOption<T>, value: T) -> AVLNodeOption<T> {
            match &avl_node {
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
                // If AVL doesn't exist create a new node as root or it's reached
                // when there's no any child node so we can insert a new node.
                _ => avl_node = AVLNode::new(value, None, None, None, 0),
            }

            // Get the balance
            let mut balance = 0;
            match &avl_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    balance = left_height - right_height;
                }
                _ => (),
            }

            if balance == 2 {
                // Get left subtree's height
                let mut left_balance = 0;
                match &avl_node {
                    Some(node) => match &node.borrow().left {
                        Some(left) => {
                            let left_height =
                                match self.get_node_height(match &left.borrow().left {
                                    Some(left_left) => Some(Rc::clone(left_left)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            let right_height =
                                match self.get_node_height(match &left.borrow().right {
                                    Some(left_right) => Some(Rc::clone(left_right)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            left_balance = left_height - right_height;
                        }
                        _ => (),
                    },
                    _ => (),
                }

                if left_balance != 1 {
                    match &avl_node {
                        Some(node) => {
                            let left = self.rotate_left(match &node.borrow().left {
                                Some(left) => Some(Rc::clone(left)),
                                _ => None,
                            });
                            node.borrow_mut().left = left;
                        }
                        _ => (),
                    }
                }
                avl_node = self.rotate_right(match &avl_node {
                    Some(node) => Some(Rc::clone(node)),
                    _ => None,
                })
            } else if balance == -2 {
                // Get right subtree's height
                let mut right_balance = 0;
                match &avl_node {
                    Some(node) => match &node.borrow().right {
                        Some(right) => {
                            let left_height =
                                match self.get_node_height(match &right.borrow().left {
                                    Some(right_left) => Some(Rc::clone(right_left)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            let right_height =
                                match self.get_node_height(match &right.borrow().right {
                                    Some(right_right) => Some(Rc::clone(right_right)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            right_balance = left_height - right_height;
                        }
                        _ => (),
                    },
                    _ => (),
                }

                if right_balance != -1 {
                    match &avl_node {
                        Some(node) => {
                            let right = self.rotate_right(match &node.borrow().right {
                                Some(right) => Some(Rc::clone(right)),
                                _ => None,
                            });
                            node.borrow_mut().right = right;
                        }
                        _ => (),
                    }
                }
                avl_node = self.rotate_left(match &avl_node {
                    Some(node) => Some(Rc::clone(node)),
                    _ => None,
                })
            }

            // Refresh the node's height
            match &avl_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };

                    node.borrow_mut().height = cmp::max(left_height, right_height) + 1;
                }
                _ => (),
            }

            avl_node
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

        fn print_inorder_node(&self, avl_node: AVLNodeOption<T>) {
            match &avl_node {
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

        fn search_node(&self, avl_node: AVLNodeOption<T>, value: T) -> AVLNodeOption<T> {
            match &avl_node {
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

        fn find_min_value(&self, avl_node: AVLNodeOption<T>) -> Option<T> {
            match &avl_node {
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

        fn find_max_value(&self, avl_node: AVLNodeOption<T>) -> Option<T> {
            match &avl_node {
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

        fn successor_value(&self, avl_node: AVLNodeOption<T>) -> Option<T> {
            match &avl_node {
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

        fn predecessor_value(&self, avl_node: AVLNodeOption<T>) -> Option<T> {
            match &avl_node {
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

        fn remove_node(&self, mut avl_node: AVLNodeOption<T>, value: T) -> AVLNodeOption<T> {
            match &avl_node {
                Some(node) => {
                    let mut node_value = node.borrow().value;
                    // Target node is found
                    if node_value == value {
                        let mut have_two_children: bool = false;
                        let mut right_node: AVLNodeOption<T> = None;
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
                        avl_node = match &new_node {
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
                }
                _ => avl_node = None,
            }

            // Get the balance
            let mut balance = 0;
            match &avl_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    balance = left_height - right_height;
                }
                _ => (),
            }

            if balance == 2 {
                // Get left subtree's height
                let mut left_balance = 0;
                match &avl_node {
                    Some(node) => match &node.borrow().left {
                        Some(left) => {
                            let left_height =
                                match self.get_node_height(match &left.borrow().left {
                                    Some(left_left) => Some(Rc::clone(left_left)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            let right_height =
                                match self.get_node_height(match &left.borrow().right {
                                    Some(left_right) => Some(Rc::clone(left_right)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            left_balance = left_height - right_height;
                        }
                        _ => (),
                    },
                    _ => (),
                }

                if left_balance != 1 {
                    match &avl_node {
                        Some(node) => {
                            let left = self.rotate_left(match &node.borrow().left {
                                Some(left) => Some(Rc::clone(left)),
                                _ => None,
                            });
                            node.borrow_mut().left = left;
                        }
                        _ => (),
                    }
                }
                avl_node = self.rotate_right(match &avl_node {
                    Some(node) => Some(Rc::clone(node)),
                    _ => None,
                })
            } else if balance == -2 {
                // Get right subtree's height
                let mut right_balance = 0;
                match &avl_node {
                    Some(node) => match &node.borrow().right {
                        Some(right) => {
                            let left_height =
                                match self.get_node_height(match &right.borrow().left {
                                    Some(right_left) => Some(Rc::clone(right_left)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            let right_height =
                                match self.get_node_height(match &right.borrow().right {
                                    Some(right_right) => Some(Rc::clone(right_right)),
                                    _ => None,
                                }) {
                                    Some(value) => value,
                                    _ => 0,
                                };
                            right_balance = left_height - right_height;
                        }
                        _ => (),
                    },
                    _ => (),
                }

                if right_balance != -1 {
                    match &avl_node {
                        Some(node) => {
                            let right = self.rotate_right(match &node.borrow().right {
                                Some(right) => Some(Rc::clone(right)),
                                _ => None,
                            });
                            node.borrow_mut().right = right;
                        }
                        _ => (),
                    }
                }
                avl_node = self.rotate_left(match &avl_node {
                    Some(node) => Some(Rc::clone(node)),
                    _ => None,
                })
            }

            // Refresh the node's height
            match &avl_node {
                Some(node) => {
                    let left_height = match self.get_node_height(match &node.borrow().left {
                        Some(left) => Some(Rc::clone(left)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };
                    let right_height = match self.get_node_height(match &node.borrow().right {
                        Some(right) => Some(Rc::clone(right)),
                        _ => None,
                    }) {
                        Some(value) => value,
                        _ => 0,
                    };

                    node.borrow_mut().height = cmp::max(left_height, right_height) + 1;
                }
                _ => (),
            }

            avl_node
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

    impl<T> Drop for AVLTree<T> {
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
