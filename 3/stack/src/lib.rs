pub mod stack {
    use node::node::*;

    use std::fmt::Display;
    use std::rc::Rc;

    #[derive(PartialEq, Debug)]
    pub struct Stack<T> {
        count: usize,
        top: NodeOption<T>,
    }

    impl<T: Clone + Display> Stack<T> {
        pub fn new() -> Self {
            Self {
                count: 0,
                top: None,
            }
        }

        pub fn top(&self) -> Option<T> {
            match &self.top {
                Some(node) => Some(node.borrow().value.clone()),
                _ => None,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        pub fn push(&mut self, value: T) {
            let new_node = Node::new(
                value,
                match &self.top {
                    Some(node) => Some(Rc::clone(node)),
                    _ => None,
                },
            );

            self.top = new_node;
            self.count += 1;
        }

        pub fn pop(&mut self) {
            if self.is_empty() {
                return;
            }

            self.top = match &self.top {
                Some(node) => match &node.borrow().next {
                    Some(prev) => Some(Rc::clone(prev)),
                    _ => None,
                },
                _ => None,
            };

            self.count -= 1;
        }

        pub fn print(&mut self) {
            match &self.top {
                Some(top) => {
                    for node in NodeIterator::new(Some(Rc::clone(top))) {
                        print!("{} -> ", node.borrow().value);
                    }
                }
                _ => (),
            }
            println!("End")
        }
    }
}
