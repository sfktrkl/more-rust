use deque::deque::*;

fn main() {
    let mut deque = Deque::<i32>::new();
    deque.enqueue_back(26);
    deque.enqueue_back(78);
    deque.enqueue_back(44);
    deque.enqueue_front(91);
    deque.enqueue_front(35);
    deque.enqueue_back(12);
    deque.print();

    while !deque.is_empty() {
        deque.dequeue_back();
    }
    deque.print();
}
