pub mod queue {
    use node::node::*;

    use std::fmt::Display;
    use std::rc::Rc;

    #[derive(PartialEq, Debug)]
    pub struct Queue<T> {
        count: usize,
        front: NodeOption<T>,
        back: NodeOption<T>,
    }

    impl<T: Clone + Display> Queue<T> {
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

        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        pub fn enqueue(&mut self, value: T) {
            let new_node = Node::new(value, None);

            if self.is_empty() {
                self.front = match &new_node {
                    Some(new) => Some(Rc::clone(new)),
                    _ => None,
                };
            } else {
                match &self.back {
                    Some(node) => {
                        node.borrow_mut().next = match &new_node {
                            Some(new) => Some(Rc::clone(new)),
                            _ => None,
                        }
                    }
                    _ => (),
                }
            }

            self.back = new_node;
            self.count += 1;
        }

        pub fn dequeue(&mut self) {
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

            if self.count == 1 {
                self.back = None;
            }

            self.count -= 1;
        }

        pub fn print(&mut self) {
            match &self.front {
                Some(front) => {
                    for node in NodeIterator::new(Some(Rc::clone(front))) {
                        print!("{} -> ", node.borrow().value);
                    }
                }
                _ => (),
            }
            println!("End")
        }
    }
}
