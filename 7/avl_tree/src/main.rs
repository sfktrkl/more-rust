use avl_tree::avl_tree::*;

fn main() {
    let mut avl = AVLTree::new();
    let array = [23, 12, 31, 3, 15, 7, 29, 88, 53];

    for i in 0..array.len() {
        avl.insert(array[i]);
    }
    print!("Values: ");
    avl.print_inorder();

    println!("\nSearch value 31: {}", avl.search(31));
    println!("Search value 18: {}", avl.search(18));

    println!("\nMinimum value: {:?}", avl.find_min());
    println!("Maximum value: {:?}", avl.find_max());

    println!("\nSuccessor(31): {:?}", avl.successor(31));
    println!("Successor(15): {:?}", avl.successor(15));
    println!("Successor(88): {:?}", avl.successor(88));

    println!("\nPredecessor(12): {:?}", avl.predecessor(12));
    println!("Predecessor(29): {:?}", avl.predecessor(29));
    println!("Predecessor(3): {:?}", avl.predecessor(3));

    println!("\nRemove 15");
    avl.remove(15);
    print!("Values: ");
    avl.print_inorder();

    println!("Remove 53");
    avl.remove(53);
    print!("Values: ");
    avl.print_inorder();
}
