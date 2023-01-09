use node::node::*;
use std::fmt::Display;

fn print_node<T: Display>(node: NodeOption<T>) {
    for node in NodeIterator::new(node) {
        print!("{} -> ", node.borrow().value);
    }
    println!("None")
}

fn main() {
    let node3 = Node::new(21, None);
    let node2 = Node::new(14, node3);
    let node1 = Node::new(7, node2);
    print_node::<i32>(node1);

    let nodef3 = Node::new(8.17, None);
    let nodef2 = Node::new(6.45, nodef3);
    let nodef1 = Node::new(4.93, nodef2);
    print_node::<f64>(nodef1);
}
