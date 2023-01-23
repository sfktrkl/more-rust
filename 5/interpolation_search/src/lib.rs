pub mod interpolation_search {
    fn search(array: &[i32], value: i32, start_index: usize, end_index: usize) -> Option<usize> {
        if start_index <= end_index {
            let middle_index = start_index
                + ((value - array[start_index]) as usize * (end_index - start_index)
                    / (array[end_index] - array[start_index]) as usize);

            // If the middle index's value is the
            // searched value return the index.
            if array[middle_index] == value {
                return Some(middle_index);
            // If the middle index's value higher
            // than the searched value, perform
            // binary search over left subarray.
            } else if array[middle_index] > value {
                return search(array, value, start_index, middle_index - 1);
            // If the middle index's value lower
            // than the searched value, perform
            // binary search over right subarray.
            } else {
                return search(array, value, middle_index + 1, end_index);
            }
        }
        return None;
    }

    pub fn interpolation_search(array: &[i32], value: i32) -> (String, Option<usize>) {
        let index = search(array, value, 0, array.len() - 1);
        return (
            String::from("Ω(1)-O(log(log(n))) Interpolation Search"),
            index,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::interpolation_search::interpolation_search;
    use helper::helper::create_array;

    #[test]
    fn test_interpolation_search_sorted() {
        let value = 30;
        // Create array
        let array = create_array(true);
        let clone = &array.clone();

        // Search and test
        assert_eq!(
            array.iter().position(|&x| x == value).unwrap(),
            interpolation_search(clone, value).1.unwrap()
        );
    }
}
