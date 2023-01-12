use doubly_node::doubly_node::*;
use std::fmt::Display;
use std::rc::Rc;

fn print_forward<T: Display>(node: DoublyNodeOption<T>) {
    for node in DoublyNodeIterator::new(node) {
        print!("{} -> ", node.borrow().value);
    }
    println!("None")
}

fn print_backward<T: Display>(node: DoublyNodeOption<T>) {
    for node in DoublyNodeIterator::new(node).rev() {
        print!("{} -> ", node.borrow().value);
    }
    println!("None")
}

fn main() {
    let node1 = DoublyNode::new(7, None, None);
    let node2 = DoublyNode::new(14, None, None);
    let node3 = DoublyNode::new(21, None, None);

    match &node1 {
        Some(node) => {
            node.borrow_mut().next = match &node2 {
                Some(next) => Some(Rc::clone(next)),
                _ => None,
            };
        }
        _ => (),
    }

    match &node2 {
        Some(node) => {
            node.borrow_mut().next = match &node3 {
                Some(next) => Some(Rc::clone(next)),
                _ => None,
            };
            node.borrow_mut().previous = match &node1 {
                Some(prev) => Some(Rc::clone(prev)),
                _ => None,
            };
        }
        _ => (),
    }

    match &node3 {
        Some(node) => {
            node.borrow_mut().previous = match &node2 {
                Some(prev) => Some(Rc::clone(prev)),
                _ => None,
            };
        }
        _ => (),
    }

    print_forward::<i32>(node1);
    print_backward::<i32>(node3);
}
