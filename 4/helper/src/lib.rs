pub mod helper {
    use rand::{thread_rng, Rng};
    use std::time::Instant;

    pub fn sort<T>(f: fn(&mut [T]) -> String, array: &mut [T]) {
        let start = Instant::now();
        let function_name = f(array);
        let duration = start.elapsed();
        println!("Time elapsed \"{}\":\n\t{:?}\n", function_name, duration);
    }

    const ARRAY_SIZE: usize = 1000;
    pub fn create_array() -> [i32; ARRAY_SIZE] {
        let mut array = [0i32; ARRAY_SIZE];
        thread_rng().fill(&mut array[..]);
        return array;
    }
    pub fn create_array_range() -> [i32; ARRAY_SIZE] {
        let mut array = [0i32; ARRAY_SIZE];
        for x in &mut array {
            *x = thread_rng().gen_range(-1000..1000);
        }
        return array;
    }

    pub fn swap<T: Copy>(array: &mut [T], i: usize, j: usize) {
        let temp: T;
        temp = array[i];
        array[i] = array[j];
        array[j] = temp;
    }
}
