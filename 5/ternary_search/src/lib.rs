pub mod ternary_search {
    fn search<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
        array: &[T],
        value: T,
        start_index: usize,
        end_index: usize,
    ) -> Option<usize> {
        if start_index <= end_index {
            let middle_left_index = start_index + (end_index - start_index) / 3;
            let middle_right_index = middle_left_index + (end_index - start_index) / 3;

            // If the middle left index's value
            // is the value return the index.
            if array[middle_left_index] == value {
                return Some(middle_left_index);
            // If the middle right index's value
            // is the value return the index.
            } else if array[middle_right_index] == value {
                return Some(middle_right_index);
            // If the middle left index's value
            // higher than the value, perform
            // binary search over left subarray.
            } else if array[middle_left_index] > value {
                return search(array, value, start_index, middle_left_index - 1);
            // If the middle right index's value
            // lower than the value, perform
            // binary search over right subarray.
            } else if array[middle_right_index] < value {
                return search(array, value, middle_right_index + 1, end_index);
            } else {
                return search(array, value, middle_left_index + 1, middle_right_index - 1);
            }
        }
        return None;
    }

    pub fn ternary_search<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
        array: &[T],
        value: T,
    ) -> (String, Option<usize>) {
        let index = search(array, value, 0, array.len() - 1);
        return (String::from("Ω(1)-O(log₃(n)) Ternary Search"), index);
    }
}

#[cfg(test)]
mod tests {
    use super::ternary_search::ternary_search;
    use helper::helper::create_array;

    #[test]
    fn test_ternary_search_sorted() {
        let value = 30;
        // Create array
        let array = create_array(true);
        let clone = &array.clone();

        // Search and test
        assert_eq!(
            array.iter().position(|&x| x == value).unwrap(),
            ternary_search(clone, value).1.unwrap()
        );
    }
}
