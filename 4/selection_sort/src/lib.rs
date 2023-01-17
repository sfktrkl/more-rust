pub mod selection_sort {
    use helper::helper::swap;

    pub fn selection_sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        let mut min_index: usize;
        for i in 0..array.len() - 1 {
            // Set the first unsorted element as minimum value
            min_index = i;

            // Find the minimum value in unsorted array
            for j in i + 1..array.len() {
                if array[j] < array[min_index] {
                    min_index = j;
                }
            }

            // Swap the first unsorted element with the minimum value
            swap(array, i, min_index);
        }
        String::from("Ω(n²)-O(n²) Selection Sort")
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;
    use helper::helper::create_array;

    #[test]
    fn test_selection_sort() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        selection_sort::selection_sort(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }
}
