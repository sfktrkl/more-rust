use exponential_search::exponential_search::*;
use helper::helper::*;

fn main() {
    let value = 30;
    println!("\nExponential Search\n");
    search_easy(exponential_search, value);

    println!("\nTry elapsed time with 200000 elements.");
    search(exponential_search, true, &create_array(true), value);
}
