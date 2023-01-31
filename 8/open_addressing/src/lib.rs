pub mod hash_table {
    use std::fmt::Display;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    const TABLE_SIZE: usize = 7;
    #[derive(PartialEq, Debug)]
    pub struct HashTable<T, V> {
        table: [Option<(T, V)>; TABLE_SIZE as usize],
        current_size: usize,
    }

    impl<T: PartialEq + Hash + Clone + Display, V: Clone + Display> HashTable<T, V> {
        pub fn new() -> Self {
            HashTable {
                table: Default::default(),
                current_size: 0,
            }
        }

        fn hash_function(&self, key: T) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            (hasher.finish() % TABLE_SIZE as u64) as usize
        }

        pub fn insert(&mut self, key: T, value: V) {
            if self.current_size >= TABLE_SIZE {
                return;
            }
            let mut hash_key = self.hash_function(key.clone());

            // Find the next free space using linear probing.
            loop {
                match &self.table[hash_key] {
                    Some(pair) => {
                        let (current_key, _) = pair.clone();
                        if current_key != key {
                            hash_key += 1;
                            hash_key %= TABLE_SIZE;
                            continue;
                        }
                        break;
                    }
                    _ => break,
                }
            }

            self.table[hash_key] = Some((key, value));
            self.current_size += 1;
        }

        pub fn search(&mut self, key: T) -> Option<V> {
            let mut hash_key = self.hash_function(key.clone());

            let mut looped = 0;
            loop {
                match &self.table[hash_key] {
                    Some(pair) => {
                        let (current_key, current_value) = pair.clone();
                        if current_key == key {
                            return Some(current_value);
                        }
                    }
                    _ => (),
                }
                if looped > TABLE_SIZE {
                    break;
                }
                looped += 1;
                hash_key += 1;
                hash_key %= TABLE_SIZE;
            }

            return None;
        }

        pub fn remove(&mut self, key: T) {
            let mut hash_key = self.hash_function(key.clone());

            let mut looped = 0;
            loop {
                match &self.table[hash_key] {
                    Some(pair) => {
                        let (current_key, _) = pair.clone();
                        if current_key == key {
                            self.table[hash_key] = None;
                            self.current_size -= 1;
                            return;
                        }
                    }
                    _ => (),
                }
                if looped > TABLE_SIZE {
                    return;
                }
                looped += 1;
                hash_key += 1;
                hash_key %= TABLE_SIZE;
            }
        }

        pub fn is_empty(&self) -> bool {
            self.current_size == 0
        }

        pub fn print(&self) {
            for i in 0..TABLE_SIZE {
                print!("Cell {}: ", i);
                match &self.table[i] {
                    Some(pair) => {
                        let (key, value) = pair;
                        print!("{}.{}", key, value);
                    }
                    _ => print!("None"),
                }
                println!("");
            }
        }
    }
}
