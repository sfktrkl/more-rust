pub mod linear_search {
    pub fn linear_search<T: std::cmp::PartialEq>(array: &[T], value: T) -> (String, Option<usize>) {
        let mut index = None;
        // Iterate through array to find the value
        for i in 0..array.len() {
            if array[i] == value {
                index = Some(i);
                break;
            }
        }
        return (String::from("Ω(1)-O(n) Linear Search"), index);
    }
}

#[cfg(test)]
mod tests {
    use super::linear_search::linear_search;
    use helper::helper::create_array;

    #[test]
    fn test_linear_search_sorted() {
        let value = 30;
        // Create array
        let array = create_array(true);
        let clone = &array.clone();

        // Search and test
        assert_eq!(
            array.iter().position(|&x| x == value).unwrap(),
            linear_search(clone, value).1.unwrap()
        );
    }

    #[test]
    fn test_linear_search_shuffled() {
        let value = 30;
        // Create array
        let array = create_array(false);
        let clone = &array.clone();

        // Search and test
        assert_eq!(
            array.iter().position(|&x| x == value).unwrap(),
            linear_search(clone, value).1.unwrap()
        );
    }
}
