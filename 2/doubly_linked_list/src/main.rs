use doubly_linked_list::doubly_linked_list::*;

fn main() {
    let mut doubly_linked_list = DoublyLinkedList::<i32>::new();
    doubly_linked_list.insert_head(43);
    doubly_linked_list.insert_head(76);
    doubly_linked_list.insert_tail(15);
    doubly_linked_list.insert_tail(44);

    println!("First Printed: ");
    doubly_linked_list.print_forward();
    doubly_linked_list.print_backward();

    doubly_linked_list.insert(4, 100);
    doubly_linked_list.insert(3, 48);
    doubly_linked_list.insert(0, 22);

    println!("Second Printed: ");
    doubly_linked_list.print_forward();
    doubly_linked_list.print_backward();

    println!("Get value of the second index: ");
    println!(
        "{}",
        match doubly_linked_list.get(2) {
            Some(node) => node.borrow().value,
            _ => 0,
        }
    );

    println!("The position of the value 15: ");
    println!(
        "{}",
        match doubly_linked_list.search(15) {
            Some(index) => index,
            _ => usize::MAX,
        }
    );

    println!("Remove the first element: ");
    doubly_linked_list.remove_head();
    doubly_linked_list.print_forward();
    doubly_linked_list.print_backward();

    println!("Remove the fifth element: ");
    doubly_linked_list.remove(4);
    doubly_linked_list.print_forward();
    doubly_linked_list.print_backward();

    println!("Remove the last element: ");
    doubly_linked_list.remove_tail();
    doubly_linked_list.print_forward();
    doubly_linked_list.print_backward();

    println!("Remove the tenth element: ");
    doubly_linked_list.remove(9);
    doubly_linked_list.print_forward();
    doubly_linked_list.print_backward();
}
