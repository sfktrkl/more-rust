pub mod quick_sort {
    use helper::helper::swap;

    fn partition<T: std::cmp::PartialOrd + Copy>(
        array: &mut [T],
        start_index: isize,
        end_index: isize,
    ) -> isize {
        // Set the first item as pivot
        let pivot = array[start_index as usize];
        let mut middle_index = start_index;

        for i in start_index + 1..=end_index {
            if array[i as usize] < pivot {
                // current item is on the left subarray
                // prepare a seat by shifting the index
                middle_index += 1;

                // middle index is the member of the right
                // subarray, swap it to the current item
                // which is the member of the left subarray
                swap(array, i as usize, middle_index as usize);
            }
        }

        // middle index is the member of the left subarray,
        // swap it with the pivot so the pivot will be in
        // between left and right subarray
        swap(array, start_index as usize, middle_index as usize);

        return middle_index;
    }

    fn sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T], start_index: isize, end_index: isize) {
        if start_index < end_index {
            // Retrieve the pivot position
            let pivot_index = partition(array, start_index, end_index);
            // Sort the left subarray
            sort(array, start_index, pivot_index - 1);
            // Sort the right subarray
            sort(array, pivot_index + 1, end_index);
        }
    }

    pub fn quick_sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        let start_index = 0;
        let end_index = array.len() - 1;
        sort(array, start_index, end_index as isize);
        String::from("Ω(nlog(n))-O(n²) Quick Sort")
    }
}

#[cfg(test)]
mod tests {
    use super::quick_sort;
    use helper::helper::create_array;

    #[test]
    fn test_quick_sort() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        quick_sort::quick_sort(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }
}
