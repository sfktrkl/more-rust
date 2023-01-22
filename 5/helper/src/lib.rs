pub mod helper {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::time::Instant;

    pub fn search_easy<T: std::fmt::Display + std::fmt::Debug + Copy>(
        f: fn(&[i32], T) -> (String, Option<usize>),
        value: T,
    ) {
        println!(
            "Index: {}",
            "[0, 1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14]"
        );
        let array = [3, 8, 11, 15, 16, 23, 28, 30, 32, 39, 42, 44, 47, 48, 50];
        println!("Array: {:?}", array);
        let (function_name, index) = f(&array, value);
        match index {
            Some(v) => println!("{}: {} is found in index {}", function_name, value, v),
            _ => println!("{}: {} is not found.", function_name, value),
        };
    }

    pub fn search<T: std::fmt::Display + std::fmt::Debug + Copy>(
        f: fn(&[T], T) -> (String, Option<usize>),
        sorted: bool,
        array: &[T],
        value: T,
    ) {
        let start = Instant::now();
        let (function_name, index) = f(array, value);
        let duration = start.elapsed();

        match index {
            Some(v) => println!(
                "Time elapsed \"{}\" sorted {}:\n\t{} is found in: {:?} at index {:?}\n",
                function_name, sorted, value, duration, v
            ),
            _ => println!(
                "Time elapsed \"{}\" sorted {}:\n\t{} is not found in {:?}.\n",
                function_name, sorted, value, duration
            ),
        };
    }

    const ARRAY_SIZE: usize = 200000;
    pub fn create_array(sorted: bool) -> [i32; ARRAY_SIZE] {
        let mut array = [0i32; ARRAY_SIZE];
        for i in 0..ARRAY_SIZE {
            array[i] = i as i32 - (ARRAY_SIZE / 2) as i32;
        }
        if !sorted {
            array.shuffle(&mut thread_rng());
        }
        return array;
    }
}
