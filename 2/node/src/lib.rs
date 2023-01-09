pub mod node {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type NodeRef<T> = Rc<RefCell<Node<T>>>;
    pub type NodeOption<T> = Option<NodeRef<T>>;

    #[derive(PartialEq, Debug)]
    pub struct Node<T> {
        pub value: T,
        pub next: NodeOption<T>,
    }

    impl<T> Node<T> {
        pub fn new(value: T, next: NodeOption<T>) -> NodeOption<T> {
            Some(Rc::new(RefCell::new(Node {
                value: value,
                next: next,
            })))
        }
    }

    pub struct NodeIterator<T> {
        pub node: NodeOption<T>,
    }

    impl<T> NodeIterator<T> {
        pub fn new(node: NodeOption<T>) -> Self {
            NodeIterator { node: node }
        }
    }

    impl<T> Iterator for NodeIterator<T> {
        type Item = Rc<RefCell<Node<T>>>;

        fn next(&mut self) -> NodeOption<T> {
            let mut result = None;

            self.node = match &self.node {
                Some(node) => {
                    result = Some(Rc::clone(node));
                    match &node.borrow().next {
                        Some(next_node) => Some(Rc::clone(next_node)),
                        _ => None,
                    }
                }
                _ => None,
            };

            result
        }
    }
}
