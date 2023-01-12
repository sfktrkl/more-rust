pub mod doubly_node {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type DoublyNodeRef<T> = Rc<RefCell<DoublyNode<T>>>;
    pub type DoublyNodeOption<T> = Option<DoublyNodeRef<T>>;

    #[derive(PartialEq, Debug)]
    pub struct DoublyNode<T> {
        pub value: T,
        pub next: DoublyNodeOption<T>,
        pub previous: DoublyNodeOption<T>,
    }

    impl<T> DoublyNode<T> {
        pub fn new(
            value: T,
            next: DoublyNodeOption<T>,
            previous: DoublyNodeOption<T>,
        ) -> DoublyNodeOption<T> {
            Some(Rc::new(RefCell::new(DoublyNode {
                value: value,
                next: next,
                previous: previous,
            })))
        }
    }

    pub struct DoublyNodeIterator<T> {
        pub node: DoublyNodeOption<T>,
    }

    impl<T> DoublyNodeIterator<T> {
        pub fn new(node: DoublyNodeOption<T>) -> Self {
            DoublyNodeIterator { node: node }
        }
    }

    impl<T> Iterator for DoublyNodeIterator<T> {
        type Item = Rc<RefCell<DoublyNode<T>>>;

        fn next(&mut self) -> DoublyNodeOption<T> {
            let mut result = None;

            self.node = match &self.node {
                Some(node) => {
                    result = Some(Rc::clone(node));
                    match &node.borrow().next {
                        Some(next) => Some(Rc::clone(next)),
                        _ => None,
                    }
                }
                _ => None,
            };

            result
        }
    }

    impl<T> DoubleEndedIterator for DoublyNodeIterator<T> {
        fn next_back(&mut self) -> DoublyNodeOption<T> {
            let mut result = None;

            self.node = match &self.node {
                Some(node) => {
                    result = Some(Rc::clone(node));
                    match &node.borrow().previous {
                        Some(next) => Some(Rc::clone(next)),
                        _ => None,
                    }
                }
                _ => None,
            };

            result
        }
    }
}
