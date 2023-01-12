use linked_list::linked_list::*;

fn main() {
    let mut linked_list = LinkedList::<i32>::new();
    linked_list.insert_head(43);
    linked_list.insert_head(76);
    linked_list.insert_tail(15);
    linked_list.insert_tail(44);

    println!("First Printed: ");
    linked_list.print();

    linked_list.insert(4, 100);
    linked_list.insert(3, 48);
    linked_list.insert(0, 22);

    println!("Second Printed: ");
    linked_list.print();

    println!("Get value of the second index: ");
    println!(
        "{}",
        match linked_list.get(2) {
            Some(node) => node.borrow().value,
            _ => 0,
        }
    );

    println!("The position of the value 15: ");
    println!(
        "{}",
        match linked_list.search(15) {
            Some(index) => index,
            _ => usize::MAX,
        }
    );

    println!("Remove the first element: ");
    linked_list.remove_head();
    linked_list.print();

    println!("Remove the fifth element: ");
    linked_list.remove(4);
    linked_list.print();

    println!("Remove the last element: ");
    linked_list.remove_tail();
    linked_list.print();

    println!("Remove the tenth element: ");
    linked_list.remove(9);
    linked_list.print();
}
