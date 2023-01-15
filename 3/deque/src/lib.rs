pub mod deque {
    use doubly_node::doubly_node::*;

    use std::fmt::Display;
    use std::rc::Rc;

    #[derive(PartialEq, Debug)]
    pub struct Deque<T> {
        count: usize,
        front: DoublyNodeOption<T>,
        back: DoublyNodeOption<T>,
    }

    impl<T: Clone + Display> Deque<T> {
        pub fn new() -> Self {
            Self {
                count: 0,
                front: None,
                back: None,
            }
        }

        pub fn front(&self) -> Option<T> {
            match &self.front {
                Some(node) => Some(node.borrow().value.clone()),
                _ => None,
            }
        }

        pub fn back(&self) -> Option<T> {
            match &self.back {
                Some(node) => Some(node.borrow().value.clone()),
                _ => None,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        pub fn enqueue_front(&mut self, value: T) {
            let new_node = DoublyNode::new(
                value,
                match &self.front {
                    Some(node) => Some(Rc::clone(node)),
                    _ => None,
                },
                None,
            );

            match &self.front {
                Some(node) => {
                    node.borrow_mut().previous = match &new_node {
                        Some(new) => Some(Rc::clone(new)),
                        _ => None,
                    }
                }
                _ => (),
            }

            if self.is_empty() {
                self.back = match &new_node {
                    Some(new) => Some(Rc::clone(new)),
                    _ => None,
                }
            }

            self.front = new_node;
            self.count += 1;
        }

        pub fn enqueue_back(&mut self, value: T) {
            if self.is_empty() {
                self.enqueue_front(value);
                return;
            }

            let new_node = DoublyNode::new(
                value,
                None,
                match &self.back {
                    Some(node) => Some(Rc::clone(node)),
                    _ => None,
                },
            );

            match &self.back {
                Some(node) => {
                    node.borrow_mut().next = match &new_node {
                        Some(new) => Some(Rc::clone(new)),
                        _ => None,
                    }
                }
                _ => (),
            }

            self.back = new_node;
            self.count += 1;
        }

        pub fn dequeue_front(&mut self) {
            if self.is_empty() {
                return;
            }

            self.front = match &self.front {
                Some(node) => match &node.borrow().next {
                    Some(next) => Some(Rc::clone(next)),
                    _ => None,
                },
                _ => None,
            };

            match &self.front {
                Some(node) => node.borrow_mut().previous = None,
                _ => (),
            }

            if self.count == 1 {
                self.back = None;
            }

            self.count -= 1;
        }

        pub fn dequeue_back(&mut self) {
            if self.is_empty() {
                return;
            }

            if self.count == 1 {
                self.dequeue_front();
                return;
            }

            self.back = match &self.back {
                Some(node) => match &node.borrow().previous {
                    Some(next) => Some(Rc::clone(next)),
                    _ => None,
                },
                _ => None,
            };

            match &self.back {
                Some(node) => node.borrow_mut().next = None,
                _ => (),
            }

            if self.count == 1 {
                self.front = None;
            }

            self.count -= 1;
        }

        pub fn print(&mut self) {
            match &self.front {
                Some(front) => {
                    for node in DoublyNodeIterator::new(Some(Rc::clone(front))) {
                        print!("{} -> ", node.borrow().value);
                    }
                }
                _ => (),
            }
            println!("End")
        }
    }
}
