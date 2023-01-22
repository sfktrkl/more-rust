pub mod radix_sort {
    fn counting_sort(array: &mut [i32], exp: i32) {
        // Range will represent the digits.
        let range = 10;

        // Initialize the count array.
        let mut count = Vec::<i32>::with_capacity(range);
        count.resize(range, 0);
        // Store the count of that minimum element at the zero index.
        for i in 0..array.len() {
            count[((array[i] / exp) % 10) as usize] += 1;
        }

        // Store the cumulative count;
        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        let mut output = Vec::<i32>::with_capacity(array.len());
        output.resize(array.len(), 0);
        // Find the index of each element of the original array
        // in count array and place the elements to output array.
        for i in (0..array.len()).rev() {
            output[(count[((array[i] / exp) % 10) as usize] - 1) as usize] = array[i];
            count[((array[i] / exp) % 10) as usize] -= 1;
        }

        // Copy sorted elements into the original array.
        for i in 0..array.len() {
            array[i] = output[i];
        }
    }

    pub fn radix_sort(array: &mut [i32]) -> String {
        // Find the maximum number to know number of digits
        let mut max: i32 = 0;
        for i in 0..array.len() {
            if array[i] > max {
                max = array[i];
            }
        }

        let mut exp = 1;
        // Do counting sort for every digit. Note that instead
        // of passing digit number, exp is passed. exp is 10^i
        // where i is current digit number
        while max / exp > 0 {
            counting_sort(array, exp);
            exp *= 10;
        }

        String::from("Ω(nk)-O(nk) Radix Sort")
    }
}

#[cfg(test)]
mod tests {
    use super::radix_sort;
    use helper::helper::create_array_range2;

    #[test]
    fn test_radix_sort() {
        // Create array
        let mut array = create_array_range2();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        radix_sort::radix_sort(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }
}
