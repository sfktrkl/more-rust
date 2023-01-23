use helper::helper::*;
use interpolation_search::interpolation_search::*;

fn main() {
    let value = 30;
    println!("\nInterpolation Search\n");
    search_easy(interpolation_search, value);

    println!("\nTry elapsed time with 200000 elements.");
    search(interpolation_search, true, &create_array(true), value);
}
