use bst_node::bst_node::*;
use std::rc::Rc;

fn main() {
    let right_right = BSTNode::new(6, None, None, None);
    let right = BSTNode::new(
        3,
        None,
        match &right_right {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        None,
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

    let left_right = BSTNode::new(5, None, None, None);
    let left_left = BSTNode::new(4, None, None, None);
    let left = BSTNode::new(
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

    let root = BSTNode::new(
        1,
        match &left {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        match &right {
            Some(node) => Some(Rc::clone(node)),
            _ => None,
        },
        None,
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
