pub mod binary_heap {
    #[derive(PartialEq, Debug)]
    pub struct BinaryHeap<T> {
        pub vector: Vec<T>,
        pub heap_size: usize,
    }

    impl<T: std::cmp::PartialOrd + Copy + std::fmt::Debug> BinaryHeap<T> {
        pub fn new() -> Self {
            BinaryHeap {
                vector: Vec::<T>::new(),
                heap_size: 0,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.heap_size == 0
        }

        fn swap(&mut self, i: usize, j: usize) {
            let temp: T;
            temp = self.vector[i];
            self.vector[i] = self.vector[j];
            self.vector[j] = temp;
        }

        fn parent(&self, index: usize) -> usize {
            index >> 1
        }

        fn left(&self, index: usize) -> usize {
            index << 1
        }

        fn right(&self, index: usize) -> usize {
            (index << 1) + 1
        }

        fn shift_up(&mut self, index: usize) {
            if index == 0 {
                return;
            }

            if self.vector[index] > self.vector[self.parent(index)] {
                self.swap(index, self.parent(index));
                self.shift_up(self.parent(index));
            }
        }

        pub fn insert(&mut self, value: T) {
            if self.heap_size >= self.vector.len() {
                self.vector.push(value);
            } else {
                self.vector[self.heap_size] = value;
            }
            self.shift_up(self.heap_size);
            self.heap_size += 1;
        }

        fn shift_down(&mut self, index: usize) {
            if index + 1 >= self.heap_size {
                return;
            }

            let mut swap_id = index;

            // Compare with left child, if exists
            if self.left(index) <= self.heap_size - 1
                && self.vector[index] < self.vector[self.left(index)]
            {
                swap_id = self.left(index);
            }

            // Compare with right child, if exists
            if self.right(index) <= self.heap_size - 1
                && self.vector[swap_id] < self.vector[self.right(index)]
            {
                swap_id = self.right(index);
            }

            // Swap the larger of the two children
            if swap_id != index {
                self.swap(index, swap_id);
                self.shift_down(swap_id);
            }
        }

        pub fn extract_max(&mut self) -> T {
            let maximum = self.vector[0];
            self.swap(0, self.heap_size - 1);
            self.heap_size -= 1;
            //self.vector.pop();
            self.shift_down(0);
            maximum
        }

        pub fn get_max(&self) -> T {
            self.vector[0]
        }
    }
}
