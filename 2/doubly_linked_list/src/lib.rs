pub mod doubly_linked_list {
    use doubly_node::doubly_node::*;
    use std::cmp::PartialEq;
    use std::fmt::Display;
    use std::rc::Rc;

    #[derive(PartialEq, Debug)]
    pub struct DoublyLinkedList<T> {
        count: usize,
        head: DoublyNodeOption<T>,
        tail: DoublyNodeOption<T>,
    }

    impl<T: Display + PartialEq> DoublyLinkedList<T> {
        pub fn new() -> Self {
            DoublyLinkedList {
                count: 0,
                head: None,
                tail: None,
            }
        }

        pub fn get(&mut self, index: usize) -> DoublyNodeOption<T> {
            if index > self.count {
                return None;
            }

            match &self.head {
                Some(head) => {
                    let mut i = 0;
                    for node in DoublyNodeIterator::new(Some(Rc::clone(head))) {
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
            let new_node = DoublyNode::new(
                value,
                match &self.head {
                    Some(head) => Some(Rc::clone(head)),
                    _ => None,
                },
                None,
            );

            match &self.head {
                Some(head) => {
                    head.borrow_mut().previous = match &new_node {
                        Some(node) => Some(Rc::clone(node)),
                        _ => None,
                    }
                }
                _ => (),
            }

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

            let new_node = DoublyNode::new(
                value,
                None,
                match &self.tail {
                    Some(prev) => Some(Rc::clone(prev)),
                    _ => None,
                },
            );

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
                    for node in DoublyNodeIterator::new(Some(Rc::clone(head))) {
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

            let new_node = DoublyNode::new(
                value,
                match &next_node {
                    Some(next) => Some(Rc::clone(next)),
                    _ => None,
                },
                match &prev_node {
                    Some(prev) => Some(Rc::clone(prev)),
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

            match &next_node {
                Some(next) => {
                    next.borrow_mut().previous = match &new_node {
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
                    for node in DoublyNodeIterator::new(Some(Rc::clone(head))) {
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

            match &self.head {
                Some(head) => head.borrow_mut().previous = None,
                _ => (),
            }

            self.count -= 1;
        }

        pub fn remove_tail(&mut self) {
            if self.count == 0 {
                return;
            }

            self.tail = match &self.tail {
                Some(tail) => match &tail.borrow().previous {
                    Some(prev) => Some(Rc::clone(prev)),
                    _ => None,
                },
                _ => None,
            };

            match &self.tail {
                Some(tail) => tail.borrow_mut().next = None,
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
                    for node in DoublyNodeIterator::new(Some(Rc::clone(head))) {
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

            match &next_node {
                Some(next) => {
                    next.borrow_mut().previous = match &prev_node {
                        Some(prev) => Some(Rc::clone(prev)),
                        _ => None,
                    }
                }
                _ => (),
            }

            self.count -= 1;
        }

        pub fn print_forward(&mut self) {
            match &self.head {
                Some(head) => {
                    for node in DoublyNodeIterator::new(Some(Rc::clone(head))) {
                        print!("{} -> ", node.borrow().value);
                    }
                    println!("None")
                }
                _ => (),
            }
        }

        pub fn print_backward(&mut self) {
            match &self.tail {
                Some(tail) => {
                    for node in DoublyNodeIterator::new(Some(Rc::clone(tail))).rev() {
                        print!("{} -> ", node.borrow().value);
                    }
                    println!("None")
                }
                _ => (),
            }
        }
    }

    impl<T> Drop for DoublyLinkedList<T> {
        fn drop(&mut self) {
            while let Some(node) = self.head.take() {
                let _ = node.borrow_mut().previous.take();
                self.head = node.borrow_mut().next.take();
            }
            self.tail.take();
        }
    }
}
