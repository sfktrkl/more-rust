pub mod counting_sort {
    pub fn counting_sort(array: &mut [i32]) -> String {
        // Find the maximum and minimum element
        // to decide about count array's length.
        let mut max: i32 = 0;
        let mut min: i32 = 0;
        for i in 0..array.len() {
            if array[i] > max {
                max = array[i];
            }
            if array[i] < min {
                min = array[i];
            }
        }

        // Initialize the count array.
        let range = (max as isize - min as isize + 1) as usize;
        let mut count = Vec::<i32>::with_capacity(range);
        count.resize(range, 0);
        // Store the count of that minimum element at the zero index.
        for i in 0..array.len() {
            count[(array[i] - min) as usize] += 1;
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
            output[(count[(array[i] - min) as usize] - 1) as usize] = array[i];
            count[(array[i] - min) as usize] -= 1;
        }

        // Copy sorted elements into the original array.
        for i in 0..array.len() {
            array[i] = output[i];
        }

        String::from("Ω(n+k)-O(n+k) Counting Sort")
    }
}

#[cfg(test)]
mod tests {
    use super::counting_sort;
    use helper::helper::create_array_range;

    #[test]
    fn test_counting_sort() {
        // Create array
        let mut array = create_array_range();
        let clone = &mut array.clone();

        // Sort
        array.sort();
        counting_sort::counting_sort(clone);

        // Test
        for i in 0..array.len() {
            assert_eq!(array[i], clone[i]);
        }
    }
}
