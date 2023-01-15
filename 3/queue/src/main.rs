use queue::queue::*;

fn main() {
    let mut queue = Queue::<i32>::new();
    queue.enqueue(35);
    queue.enqueue(91);
    queue.enqueue(26);
    queue.enqueue(78);
    queue.enqueue(44);
    queue.enqueue(12);
    queue.print();

    while !queue.is_empty() {
        queue.dequeue();
    }
    queue.print();
}
