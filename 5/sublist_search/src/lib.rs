pub mod sublist_search {
    fn compare_matched_elements<T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug>(
        first_array: &[T],
        first_index: usize,
        second_array: &[T],
        second_index: usize,
    ) -> bool {
        if second_index >= second_array.len() {
            return true;
        }

        if first_array[first_index] == second_array[second_index] {
            return compare_matched_elements(
                first_array,
                first_index + 1,
                second_array,
                second_index + 1,
            );
        }

        return false;
    }

    pub fn search<T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug>(
        first_array: &[T],
        first_index: usize,
        second_array: &[T],
        second_index: usize,
    ) -> Option<usize> {
        if first_index < first_array.len() {
            if first_array[first_index] == second_array[second_index] {
                if compare_matched_elements(first_array, first_index, second_array, second_index) {
                    return Some(first_index);
                }
            }
            return search(first_array, first_index + 1, second_array, second_index);
        }
        return None;
    }

    pub fn sublist_search<T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug>(
        first_array: &[T],
        second_array: &[T],
    ) -> (String, Option<usize>) {
        let index;
        if first_array.len() == 0 && second_array.len() == 0 {
            index = Some(0);
        } else if first_array.len() == 0 && second_array.len() > 0 {
            index = None;
        } else if first_array.len() > 0 && second_array.len() == 0 {
            index = None;
        } else {
            index = search(first_array, 0, second_array, 0);
        }
        return (String::from("Ω(1)-O(mn) Sublist Search"), index);
    }
}

#[cfg(test)]
mod tests {
    use super::sublist_search::sublist_search;
    use helper::helper::create_array;

    #[test]
    fn test_sublist_search_sorted() {
        let start_index = 10;
        let array = create_array(true);

        // Search and test
        assert_eq!(
            start_index,
            sublist_search(&array, &array[start_index..14]).1.unwrap()
        );
    }
}
