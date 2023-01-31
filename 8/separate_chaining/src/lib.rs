pub mod hash_table {
    use std::fmt::Display;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    const TABLE_SIZE: usize = 7;
    #[derive(PartialEq, Debug)]
    pub struct HashTable<T, V> {
        table: [Vec<(T, V)>; TABLE_SIZE as usize],
    }

    impl<T: PartialEq + Hash + Clone + Display, V: Clone + Display> HashTable<T, V> {
        pub fn new() -> Self {
            HashTable {
                table: Default::default(),
            }
        }

        fn hash_function(&self, key: T) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            (hasher.finish() % TABLE_SIZE as u64) as usize
        }

        pub fn insert(&mut self, key: T, value: V) {
            let mut is_key_found: bool = false;
            let hash_key = self.hash_function(key.clone());

            for (table_key, table_value) in self.table[hash_key].iter_mut() {
                if *table_key == key {
                    is_key_found = true;
                    *table_value = value.clone();
                    break;
                }
            }

            if !is_key_found {
                self.table[hash_key].push((key, value));
            }
        }

        pub fn search(&mut self, key: T) -> Option<V> {
            let hash_key = self.hash_function(key.clone());

            for (table_key, table_value) in self.table[hash_key].iter_mut() {
                if *table_key == key {
                    return Some(table_value.clone());
                }
            }

            return None;
        }

        pub fn remove(&mut self, key: T) {
            let mut is_key_found: bool = false;
            let hash_key = self.hash_function(key.clone());

            let mut index = 0;
            for (table_key, _table_value) in self.table[hash_key].iter() {
                if *table_key == key {
                    is_key_found = true;
                    break;
                }
                index += 1;
            }

            if is_key_found {
                self.table[hash_key].remove(index);
            }
        }

        pub fn is_empty(&self) -> bool {
            let mut total_elements = 0;

            for i in 0..TABLE_SIZE {
                total_elements += self.table[i].len();
                if total_elements > 0 {
                    return false;
                }
            }

            return true;
        }

        pub fn print(&self) {
            for i in 0..TABLE_SIZE {
                print!("Cell {}: ", i);
                for j in 0..self.table[i].len() {
                    let (key, value) = &self.table[i][j];
                    print!("{}.{} -> ", key, value);
                }
                println!("None");
            }
        }
    }
}
