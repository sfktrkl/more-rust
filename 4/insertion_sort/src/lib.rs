pub mod insertion_sort {
    use helper::helper::swap;

    pub fn insertion_sort_easy<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        for i in 1..array.len() {
            let mut j = i;
            while j > 0 && array[j] < array[j - 1] {
                swap(array, j, j - 1);
                j -= 1;
            }
        }
        String::from("Ω(n)-O(n²) Insertion Sort Easy")
    }

    pub fn insertion_sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) -> String {
        for i in 0..array.len() {
            // Reference value
            let value = array[i];
            let mut j: i32 = i as i32 - 1;

            // If the value of the current index is greater than
            // the reference value then move the current value
            // to the right side otherwise, put the reference
            // value to the current index.
            while j >= 0 && array[j as usize] > value {
                array[(j + 1) as usize] = array[j as usize];
                j -= 1;
            }

            // Put the reference value to the current index
            array[(j + 1) as usize] = value;
        }
        String::from("Ω(n)-O(n²) Insertion Sort")
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;
    use helper::helper::create_array;

    #[test]
    fn test_insertion_sort() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        insertion_sort::insertion_sort(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }

    #[test]
    fn test_insertion_sort_easy() {
        // Create array
        let mut array = create_array();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        insertion_sort::insertion_sort_easy(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }
}
