pub mod exponential_search {
    use linear_search::linear_search::*;
    use std::cmp::min;

    pub fn exponential_search<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
        array: &[T],
        value: T,
    ) -> (String, Option<usize>) {
        let mut index = None;
        if array.len() > 0 {
            // Start comparing from the index 1
            let mut block_index: usize = 1;
            println!("{:?}", block_index / 2);

            // Increase the block_index exponentially,
            // if the block_index's value is lower than
            // the searched value.
            while block_index < array.len() && array[block_index] < value {
                block_index *= 2;
            }

            // When block index is found perform linear
            // search over the subarray defined by the
            // block index.
            println!("{:?}", block_index / 2);
            let start_index = block_index / 2;
            let end_index = min(block_index, array.len() - 1);
            let (_, i) = linear_search(&array[start_index..=end_index], value);
            index = match i {
                Some(value) => Some(value + start_index),
                _ => None,
            };
        }

        return (String::from("Ω(1)-O(log(n)) Exponential Search"), index);
    }
}

#[cfg(test)]
mod tests {
    use super::exponential_search::exponential_search;
    use helper::helper::create_array;

    #[test]
    fn test_exponential_search_sorted() {
        let value = 30;
        // Create array
        let array = create_array(true);
        let clone = &array.clone();

        // Search and test
        assert_eq!(
            array.iter().position(|&x| x == value).unwrap(),
            exponential_search(clone, value).1.unwrap()
        );
    }
}
