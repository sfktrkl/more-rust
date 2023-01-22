use binary_search::binary_search::*;
use helper::helper::*;

fn main() {
    let value = 30;
    println!("\nBinary Search\n");
    search_easy(binary_search, value);

    println!("\nTry elapsed time with 200000 elements.");
    search(binary_search, true, &create_array(true), value);
}
