pub mod bubble_sort {
    use helper::helper::swap;

    pub fn bubble_sort_easy<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        for i in 0..array.len() - 1 {
            for j in 0..array.len() - i - 1 {
                if array[j] > array[j + 1] {
                    swap(array, j, j + 1);
                }
            }
        }
        String::from("Ω(n)-O(n²) Bubble Sort Easy")
    }

    pub fn bubble_sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        let mut is_swapped: bool;
        let mut sorted_elements = 0;
        loop {
            is_swapped = false;

            // Compare each pair of adjacent elements
            for j in 0..array.len() - sorted_elements - 1 {
                if array[j] > array[j + 1] {
                    swap(array, j, j + 1);
                    is_swapped = true;
                }
            }

            // After each iteration the last element has been sorted.
            // So, it will be ignored in the next iteration.
            sorted_elements += 1;
            // If no elements are swapped, then array has been sorted.
            if !is_swapped {
                break;
            }
        }
        String::from("Ω(n)-O(n²) Bubble Sort")
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
    use helper::helper::create_array;

    #[test]
    fn test_bubble_sort() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        bubble_sort::bubble_sort(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }

    #[test]
    fn test_bubble_sort_easy() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        bubble_sort::bubble_sort_easy(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }
}
