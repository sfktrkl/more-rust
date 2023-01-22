use helper::helper::*;
use linear_search::linear_search::*;

fn main() {
    let value = 30;
    println!("\nLinear Search\n");
    search_easy(linear_search, value);

    println!("\nTry elapsed time with 200000 elements.");
    search(linear_search, true, &create_array(true), value);
    search(linear_search, false, &create_array(false), value);
}
