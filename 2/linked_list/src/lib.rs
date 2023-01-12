pub mod linked_list {
    use node::node::*;
    use std::cmp::PartialEq;
    use std::fmt::Display;
    use std::rc::Rc;

    #[derive(PartialEq, Debug)]
    pub struct LinkedList<T> {
        count: usize,
        head: NodeOption<T>,
        tail: NodeOption<T>,
    }

    impl<T: Display + PartialEq> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList {
                count: 0,
                head: None,
                tail: None,
            }
        }

        pub fn get(&mut self, index: usize) -> NodeOption<T> {
            if index > self.count {
                return None;
            }

            match &self.head {
                Some(head) => {
                    let mut i = 0;
                    for node in NodeIterator::new(Some(Rc::clone(head))) {
                        if i == index {
                            return Some(node);
                        }
                        i += 1;
                    }
                }
                _ => (),
            }

            return None;
        }

        pub fn insert_head(&mut self, value: T) {
            let new_node = Node::new(
                value,
                match &self.head {
                    Some(head) => Some(Rc::clone(head)),
                    _ => None,
                },
            );

            if self.count == 0 {
                self.tail = match &new_node {
                    Some(new) => Some(Rc::clone(new)),
                    _ => None,
                };
            }

            self.head = new_node;
            self.count += 1;
        }

        pub fn insert_tail(&mut self, value: T) {
            if self.count == 0 {
                self.insert_head(value);
                return;
            }

            let new_node = Node::new(value, None);

            match &self.tail {
                Some(tail) => {
                    tail.borrow_mut().next = match &new_node {
                        Some(new) => Some(Rc::clone(new)),
                        _ => None,
                    }
                }
                _ => (),
            }

            self.tail = new_node;
            self.count += 1;
        }

        pub fn insert(&mut self, index: usize, value: T) {
            if index > self.count {
                return;
            }

            if index == 0 {
                self.insert_head(value);
                return;
            } else if index == self.count {
                self.insert_tail(value);
                return;
            }

            let mut i = 0;
            let mut prev_node = None;
            let mut next_node = None;
            match &self.head {
                Some(head) => {
                    for node in NodeIterator::new(Some(Rc::clone(head))) {
                        if i == index - 1 {
                            prev_node = Some(node);
                        } else if i == index {
                            next_node = Some(node);
                            break;
                        }
                        i += 1;
                    }
                }
                _ => (),
            }

            let new_node = Node::new(
                value,
                match &next_node {
                    Some(next) => Some(Rc::clone(next)),
                    _ => None,
                },
            );

            match &prev_node {
                Some(prev) => {
                    prev.borrow_mut().next = match &new_node {
                        Some(new) => Some(Rc::clone(new)),
                        _ => None,
                    }
                }
                _ => (),
            };

            self.count += 1;
        }

        pub fn search(&self, value: T) -> Option<usize> {
            match &self.head {
                Some(head) => {
                    let mut i = 0;
                    for node in NodeIterator::new(Some(Rc::clone(head))) {
                        if node.borrow().value == value {
                            return Some(i);
                        }
                        i += 1;
                    }
                }
                _ => return None,
            }
            return None;
        }

        pub fn remove_head(&mut self) {
            if self.count == 0 {
                return;
            }

            self.head = match &self.head {
                Some(head) => match &head.borrow().next {
                    Some(next) => Some(Rc::clone(next)),
                    _ => None,
                },
                _ => None,
            };

            self.count -= 1;
        }

        pub fn remove_tail(&mut self) {
            if self.count == 0 {
                return;
            }

            if self.count == 1 {
                self.remove_head();
                return;
            }

            let mut i = 0;
            match &self.head {
                Some(head) => {
                    for node in NodeIterator::new(Some(Rc::clone(head))) {
                        if i == self.count - 2 {
                            node.borrow_mut().next = None;
                            self.tail = Some(node);
                            break;
                        }
                        i += 1;
                    }
                }
                _ => (),
            }

            self.count -= 1;
        }

        pub fn remove(&mut self, index: usize) {
            if self.count == 0 {
                return;
            }

            if index > self.count {
                return;
            }

            if index == 0 {
                self.remove_head();
                return;
            } else if index == self.count - 1 {
                self.remove_tail();
                return;
            }

            let mut i = 0;
            let mut prev_node = None;
            let mut next_node = None;
            match &self.head {
                Some(head) => {
                    for node in NodeIterator::new(Some(Rc::clone(head))) {
                        if i == index - 1 {
                            prev_node = Some(node);
                        } else if i == index + 1 {
                            next_node = Some(node);
                            break;
                        }
                        i += 1;
                    }
                }
                _ => (),
            }

            match &prev_node {
                Some(prev) => {
                    prev.borrow_mut().next = match &next_node {
                        Some(next) => Some(Rc::clone(next)),
                        _ => None,
                    }
                }
                _ => (),
            }

            self.count -= 1;
        }

        pub fn print(&mut self) {
            match &self.head {
                Some(head) => {
                    for node in NodeIterator::new(Some(Rc::clone(head))) {
                        print!("{} -> ", node.borrow().value);
                    }
                    println!("None")
                }
                _ => (),
            }
        }
    }
}
