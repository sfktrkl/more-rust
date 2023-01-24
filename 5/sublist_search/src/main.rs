use helper::helper::*;
use std::time::Instant;
use sublist_search::sublist_search::*;

fn main() {
    println!("\nSublist Search\n");

    let array = [3, 8, 11, 15, 16, 23, 28, 30, 32, 39, 42, 44, 47, 48, 50];
    let subarray = [30, 31, 39, 42, 44];
    println!("Array: {:?}", array);
    println!("Subarray: {:?}", subarray);
    let (mut function_name, mut index) = sublist_search(&array, &subarray);
    match index {
        Some(v) => println!("{}: Subarray is found in index {}", function_name, v),
        _ => println!("{}: Subarray is not found.", function_name),
    };

    println!("\nTry elapsed time.");
    let sorted = false;
    let array2 = create_array(sorted);
    let start = Instant::now();
    (function_name, index) = sublist_search(&array2, &array2[1800..2310]);
    let duration = start.elapsed();

    match index {
        Some(v) => println!(
            "Time elapsed \"{}\" sorted {}:\n\tSubarray is found in: {:?} at index {:?}\n",
            function_name, sorted, duration, v
        ),
        _ => println!(
            "Time elapsed \"{}\" sorted {}:\n\tSubarray is not found in {:?}.\n",
            function_name, sorted, duration
        ),
    };
}
