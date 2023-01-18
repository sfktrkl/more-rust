pub mod merge_sort {
    fn merge_easy<T: std::cmp::PartialOrd + Copy>(
        array: &mut [T],
        start_index: usize,
        middle_index: usize,
        end_index: usize,
    ) {
        let left_array = &array[start_index..=middle_index].to_owned();
        let right_array = &array[middle_index + 1..=end_index].to_owned();

        let mut left_index = 0;
        let mut right_index = 0;

        for merged_index in start_index..=end_index {
            if left_index < left_array.len() && right_index < right_array.len() {
                if left_array[left_index] < right_array[right_index] {
                    array[merged_index] = left_array[left_index];
                    left_index += 1;
                } else {
                    array[merged_index] = right_array[right_index];
                    right_index += 1;
                }
            } else if left_index >= left_array.len() {
                array[merged_index] = right_array[right_index];
                right_index += 1;
            } else {
                array[merged_index] = left_array[left_index];
                left_index += 1;
            }
        }
    }

    fn merge<T: std::cmp::PartialOrd + Copy>(
        array: &mut [T],
        start_index: usize,
        middle_index: usize,
        end_index: usize,
    ) {
        // Number of elements that will be sorted
        let total_elements = end_index - start_index + 1;
        let mut temp = Vec::<T>::with_capacity(total_elements);

        let mut left_index = start_index;
        let mut right_index = middle_index + 1;

        while left_index <= middle_index && right_index <= end_index {
            if array[left_index] <= array[right_index] {
                // Store the left subarray's element
                // if it is lower than the right one.
                temp.push(array[left_index]);
                left_index += 1;
            } else {
                // Store the right subarray's element
                // if it is lower than the left one.
                temp.push(array[right_index]);
                right_index += 1;
            }
        }

        // If any remaining element in the left subarray
        while left_index <= middle_index {
            temp.push(array[left_index]);
            left_index += 1;
        }

        // If any remaining element in the right subarray
        while right_index <= end_index {
            temp.push(array[right_index]);
            right_index += 1;
        }

        // Copy the elements to the original array
        for i in 0..total_elements {
            array[start_index + i] = temp[i];
        }
    }

    fn sort<T: std::cmp::PartialOrd + Copy>(
        array: &mut [T],
        start_index: usize,
        end_index: usize,
        easy: bool,
    ) {
        if start_index < end_index {
            let middle_index = (start_index + end_index) / 2;
            // Sort the left subarray
            sort(array, start_index, middle_index, easy);
            // Sort the right subarray
            sort(array, middle_index + 1, end_index, easy);
            // Merge the left and right subarrays
            if easy {
                merge_easy(array, start_index, middle_index, end_index);
            } else {
                merge(array, start_index, middle_index, end_index);
            }
        }
    }

    pub fn merge_sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        let start_index = 0;
        let end_index = array.len() - 1;
        sort(array, start_index, end_index, false);
        String::from("Ω(nlog(n))-O(nlog(n)) Merge Sort")
    }

    pub fn merge_sort_easy<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        let start_index = 0;
        let end_index = array.len() - 1;
        sort(array, start_index, end_index, true);
        String::from("Ω(nlog(n))-O(nlog(n)) Merge Sort Easy")
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use helper::helper::create_array;

    #[test]
    fn test_merge_sort() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        merge_sort::merge_sort(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }

    #[test]
    fn test_merge_sort_easy() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        merge_sort::merge_sort_easy(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }
}
