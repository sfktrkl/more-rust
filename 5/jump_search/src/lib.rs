pub mod jump_search {
    use linear_search::linear_search::*;
    use std::cmp::min;

    pub fn jump_search<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
        array: &[T],
        value: T,
    ) -> (String, Option<usize>) {
        let mut index = None;
        if array.len() > 0 {
            // Define step used to jump the array
            let step: usize = (array.len() as f64).sqrt() as usize;

            // Start comparing from the first index
            let mut block_index: usize = 0;

            // Increase the block_index by step, if the
            // block_index's value is lower than the
            // searched value.
            while block_index < array.len() && array[block_index] < value {
                block_index += step;
            }

            // When block index is found perform linear
            // search over the subarray defined by the
            // block index.
            let start_index = block_index - step;
            let end_index = min(block_index, array.len() - 1);
            let (_, i) = linear_search(&array[start_index..=end_index], value);
            index = match i {
                Some(value) => Some(value + start_index),
                _ => None,
            };
        }

        return (String::from("Ω(1)-O(√n) Jump Search"), index);
    }
}

#[cfg(test)]
mod tests {
    use super::jump_search::jump_search;
    use helper::helper::create_array;

    #[test]
    fn test_jump_search_sorted() {
        let value = 30;
        // Create array
        let array = create_array(true);
        let clone = &array.clone();

        // Search and test
        assert_eq!(
            array.iter().position(|&x| x == value).unwrap(),
            jump_search(clone, value).1.unwrap()
        );
    }
}
