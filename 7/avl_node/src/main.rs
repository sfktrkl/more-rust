use avl_node::avl_node::*;
use std::rc::Rc;

fn main() {
    let right_right = AVLNode::new(6, None, None, None, 1);
    let right = AVLNode::new(
        5,
        None,
        match &right_right {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        None,
        2,
    );
    match &right_right {
        Some(node) => {
            node.borrow_mut().parent = match &right {
                Some(right) => Some(Rc::clone(right)),
                _ => None,
            }
        }
        _ => (),
    }

    let left_right = AVLNode::new(3, None, None, None, 1);
    let left_left = AVLNode::new(1, None, None, None, 1);
    let left = AVLNode::new(
        2,
        match &left_left {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        match &left_right {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        None,
        2,
    );
    match &left_left {
        Some(node) => {
            node.borrow_mut().parent = match &left {
                Some(left) => Some(Rc::clone(left)),
                _ => None,
            }
        }
        _ => (),
    }
    match &left_right {
        Some(node) => {
            node.borrow_mut().parent = match &left {
                Some(left) => Some(Rc::clone(left)),
                _ => None,
            }
        }
        _ => (),
    }

    let root = AVLNode::new(
        4,
        match &left {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        match &right {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        None,
        3,
    );
    match &left {
        Some(node) => {
            node.borrow_mut().parent = match &root {
                Some(root) => Some(Rc::clone(root)),
                _ => None,
            }
        }
        _ => (),
    }
    match &right {
        Some(node) => {
            node.borrow_mut().parent = match &root {
                Some(root) => Some(Rc::clone(root)),
                _ => None,
            }
        }
        _ => (),
    }

    match &root {
        Some(node) => {
            print!("\nPreorder print : ");
            node.borrow().preorder_print();
            print!("\nPostorder print: ");
            node.borrow().postorder_print();
            print!("\nInorder print  : ");
            node.borrow().inorder_print();
            print!("\n");
        }
        _ => (),
    }
}
