use helper::helper::*;
use ternary_search::ternary_search::*;

fn main() {
    let value = 30;
    println!("\nTernary Search\n");
    search_easy(ternary_search, value);

    println!("\nTry elapsed time with 200000 elements.");
    search(ternary_search, true, &create_array(true), value);
}
