use binary_heap::binary_heap::*;

fn main() {
    let mut priority_queue = BinaryHeap::<i32>::new();

    println!("Is empty: {:?}", priority_queue.is_empty());

    priority_queue.insert(14);
    priority_queue.insert(53);
    priority_queue.insert(8);
    priority_queue.insert(32);
    println!("\n{:?}", priority_queue);
    println!("Max: {:?}", priority_queue.get_max());

    println!("\nExtract Max: {:?}", priority_queue.extract_max());
    println!("{:?}", priority_queue);

    println!("\nExtract Max: {:?}", priority_queue.extract_max());
    println!("{:?}", priority_queue);

    println!("\nInsert 4");
    priority_queue.insert(4);
    println!("{:?}", priority_queue);

    println!("\nExtract Max: {:?}", priority_queue.extract_max());
    println!("{:?}", priority_queue);

    println!("\nInsert 59 and 46");
    priority_queue.insert(59);
    priority_queue.insert(46);
    println!("{:?}", priority_queue);
}
