use binary_search_tree::binary_search_tree::*;

fn main() {
    let mut bst = BinarySearchTree::new();
    let array = [23, 12, 31, 3, 15, 7, 29, 88, 53];

    for i in 0..array.len() {
        bst.insert(array[i]);
    }
    print!("Values: ");
    bst.print_inorder();

    println!("\nSearch value 31: {}", bst.search(31));
    println!("Search value 18: {}", bst.search(18));

    println!("\nMinimum value: {:?}", bst.find_min());
    println!("Maximum value: {:?}", bst.find_max());

    println!("\nSuccessor(31): {:?}", bst.successor(31));
    println!("Successor(15): {:?}", bst.successor(15));
    println!("Successor(88): {:?}", bst.successor(88));

    println!("\nPredecessor(12): {:?}", bst.predecessor(12));
    println!("Predecessor(29): {:?}", bst.predecessor(29));
    println!("Predecessor(3): {:?}", bst.predecessor(3));

    println!("\nRemove 15");
    bst.remove(15);
    print!("Values: ");
    bst.print_inorder();

    println!("Remove 53");
    bst.remove(53);
    print!("Values: ");
    bst.print_inorder();
}
