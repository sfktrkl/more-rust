pub mod avl_node {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type AVLNodeRef<T> = Rc<RefCell<AVLNode<T>>>;
    pub type AVLNodeOption<T> = Option<AVLNodeRef<T>>;

    #[derive(PartialEq, Debug)]
    pub struct AVLNode<T> {
        pub value: T,
        pub left: AVLNodeOption<T>,
        pub right: AVLNodeOption<T>,
        pub parent: AVLNodeOption<T>,
        pub height: i32,
    }

    impl<T: std::fmt::Debug> AVLNode<T> {
        pub fn new(
            value: T,
            left: AVLNodeOption<T>,
            right: AVLNodeOption<T>,
            parent: AVLNodeOption<T>,
            height: i32,
        ) -> AVLNodeOption<T> {
            Some(Rc::new(RefCell::new(AVLNode {
                value: value,
                left: left,
                right: right,
                parent: parent,
                height: height,
            })))
        }

        pub fn preorder_print(&self) {
            // Print all the items in the tree to which root points.
            // The item in the root is printed first, followed by the
            // items in the left subtree and then the items in the
            // right subtree.
            print!("{:?} ", self.value);
            match &self.left {
                Some(left) => left.borrow().preorder_print(),
                _ => (),
            };
            match &self.right {
                Some(right) => right.borrow().preorder_print(),
                _ => (),
            };
        }

        pub fn postorder_print(&self) {
            // Print all the items in the tree to which root points.
            // The items in the left subtree are printed first, followed
            // by the items in the right subtree and then the item in the
            // root node.
            match &self.left {
                Some(left) => left.borrow().postorder_print(),
                _ => (),
            };
            match &self.right {
                Some(right) => right.borrow().postorder_print(),
                _ => (),
            };
            print!("{:?} ", self.value);
        }

        pub fn inorder_print(&self) {
            // Print all the items in the tree to which root points.
            // The items in the left subtree are printed first, followed
            // by the item in the root node, followed by the items in
            // the right subtree.
            match &self.left {
                Some(left) => left.borrow().inorder_print(),
                _ => (),
            };
            print!("{:?} ", self.value);
            match &self.right {
                Some(right) => right.borrow().inorder_print(),
                _ => (),
            };
        }
    }
}
