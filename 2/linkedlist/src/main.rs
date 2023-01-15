use std::collections::LinkedList;

fn main() {
    let mut linked_list = LinkedList::<i32>::new();
    linked_list.push_front(43);
    linked_list.push_front(76);
    linked_list.push_back(15);
    linked_list.push_back(44);

    println!("Linked List: {:?}", linked_list);

    match linked_list.iter_mut().find(|&&mut x| x == 15) {
        Some(value) => {
            println!("15 found in the vector.");
            *value = 57;
        }
        _ => println!("15 not found in the vector."),
    }

    println!("Linked List: {:?}", linked_list);

    match linked_list.iter().position(|&x| x == 57) {
        Some(value) => println!("57 found in vector at position: {}", value),
        _ => println!("57 not found in the vector."),
    }

    linked_list.pop_front();
    linked_list.pop_back();

    println!("Linked List: {:?}", linked_list);
}
